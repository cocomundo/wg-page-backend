use actix_web::{middleware::Logger, App, HttpServer};
use wg_page_backend::api::{
    shopping_items::{
        create_shopping_item, delete_shopping_item, get_all_shopping_items, get_shopping_item,
        update_shopping_item,
    },
    user::{create_user, delete_user, get_all_users, get_user, update_user},
};

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
            .service(get_all_users)
            .service(update_user)
            .service(delete_user)
            .service(delete_shopping_item)
            .service(create_shopping_item)
            .service(get_shopping_item)
            .service(get_all_shopping_items)
            .service(update_shopping_item)
            .service(delete_shopping_item)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
