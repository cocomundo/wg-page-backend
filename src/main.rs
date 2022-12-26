use actix_web::{middleware::Logger, App, HttpServer};
use wg_page_backend::api::{
    user::{create_user, delete_user, get_all_users, get_user, update_user},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(create_user)
            .service(get_user)
            .service(get_all_users)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
