use actix_web::{web, App, HttpServer};
mod controller;
use dotenv::dotenv;
use std::env;
mod utils;
mod apikeys;
mod schema;


#[tokio::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok(); // Load .env file if it exists
    let host = env::var("HOST").unwrap_or_else(|_| {return "127.0.0.1".to_string()});
    let port = env::var("PORT").unwrap_or_else(|_| {return "8080".to_string()});
    let address = format!("{}:{}", host, port);

    let add = |a:i32, b:i32| -> i32 {a+b};

    utils::db::connect_db().await;
    println!("Connection to db successful");
    
    return HttpServer::new(|| {
        App::new()
            .route("/healthCheck", web::get().to(controller::health::health_check))
            .route("/deposit", web::post().to(controller::paymaster::deposit))
            .route("/saveKey", web::post().to(controller::paymaster::save_key))
            .route("/whitelist", web::post().to(controller::paymaster::whitelist))
    })
    .bind(address)? // Bind to local address
    .run()
    .await
}
