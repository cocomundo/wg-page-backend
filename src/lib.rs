pub mod api;
mod db;
pub mod model;

use actix_web::dev::Server;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer};
use api::user::{create_user, delete_user, get_all_users, get_user, update_user};
use env_logger::Env;
use std::net::TcpListener;

#[get("/health_check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // set info as default, if RUST_LOG is not set
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(create_user)
            .service(get_user)
            .service(get_all_users)
            .service(update_user)
            .service(delete_user)
            .service(health_check)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
