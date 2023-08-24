use std::net::TcpListener;
use wg_page_backend::run;
use wg_page_backend::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("wg-backend".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port 8000");
    run(listener)?.await
}
