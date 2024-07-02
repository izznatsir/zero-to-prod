use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::{deserialize_bool_from_anything, deserialize_number_from_string};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
    pub name: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .username(&self.username)
            .password(&self.password.expose_secret())
            .host(&self.host)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.name)
    }
}

pub enum Environment {
    LOCAL,
    PRODUCTION,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LOCAL => "local",
            Self::PRODUCTION => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::LOCAL),
            "production" => Ok(Self::PRODUCTION),
            other => Err(format!(
                "{} is not a supported environment. \
                Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or("local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("configuration/{}.toml", environment.as_str());

    // Initialise our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `condiguration.yaml`
        .add_source(config::File::new(
            "configuration/base.toml",
            config::FileFormat::Toml,
        ))
        .add_source(config::File::new(
            &environment_filename,
            config::FileFormat::Toml,
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    // Try to convert the configuration values into
    // our Settings type
    settings.try_deserialize::<Settings>()
}
