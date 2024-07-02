use sqlx::PgPool;

use ztp::configuration::get_configuration;
use ztp::startup::run;
use ztp::tracing;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing::init("ztp", "info", std::io::stdout)
        .expect("Failed to initialize tracing subscriber.");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let db_pool = PgPool::connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener =
        std::net::TcpListener::bind(address).expect("Failed to bind TCP socket address.");

    run(listener, db_pool)?.await
}
