use actix_web::{middleware::Logger, App, HttpServer};
use args::Args;
use clap::Parser;
use wg_page_backend::api::{
    shopping_items::{
        create_shopping_item, delete_shopping_item, get_all_shopping_items, get_shopping_item,
        update_shopping_item,
    },
    user::{create_user, delete_user, get_all_users, get_user, update_user},
};

mod args;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Args::parse();
    let address = args.address;

    log::info!("Starting server at {address}");

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
    .bind(address)?
    .run()
    .await
}
