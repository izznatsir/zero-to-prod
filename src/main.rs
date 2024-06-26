use sqlx::PgPool;
use ztp::configuration::get_configuration;
use ztp::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener =
        std::net::TcpListener::bind(address).expect("Failed to bind TCP socket address.");
    run(listener, db_pool)?.await
}
