use crate::routes::{health_check, subscribe};
use actix_web::{dev, web, App, HttpServer};
use sqlx::PgPool;

pub fn run(
    listener: std::net::TcpListener,
    db_pool: PgPool,
) -> Result<dev::Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
