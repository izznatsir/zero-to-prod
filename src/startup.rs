use crate::routes::{health_check, subscribe};
use actix_web::{dev, web, App, HttpServer};

pub fn run(listener: std::net::TcpListener) -> Result<dev::Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
