use actix_web::{middleware::Logger, App, HttpServer};
use wg_page_backend::api::user::{create_user, delete_user, get_all_user, get_user, update};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(create_user)
            .service(get_user)
            .service(get_all_user)
            .service(update)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
