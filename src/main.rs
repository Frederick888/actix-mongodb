use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use std::env;

mod controllers;
mod db;
mod error;

fn main() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // initialise MongoDB connection
    let _ = db::get_client();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/items", web::get().to(controllers::items))
    })
    .bind(&env::var("BIND_ADDRESS").expect("bind address is not set!"))
    .unwrap()
    .run()
    .unwrap();
}
