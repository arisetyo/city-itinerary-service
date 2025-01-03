extern crate city_itinerary_service;

use salvo::prelude::*;
use city_itinerary_service::{routes, db};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Load .env file
    db::init_db().await.expect("Failed to initialize database");

    let router = routes::create_router();
    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(router)
        .await;
}
