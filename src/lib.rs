pub mod api;
mod db;
pub mod model;

use actix_web::{middleware::Logger, App, HttpServer, HttpResponse, get};
use actix_web::dev::Server;
use std::net::TcpListener;
use api::{
    user::{create_user, delete_user, get_all_users, get_user, update_user}
};

#[get("/health_check")]
pub async fn health_check() -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub fn run(listener : TcpListener) -> Result<Server, std::io::Error> {
    env_logger::init();

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
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
