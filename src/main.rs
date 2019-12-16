#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod api;
mod db;
mod model;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    match env::var("RUST_LOG") {
        Ok(_) => {}
        Err(_) => env::set_var("RUST_LOG", "actix_web=info"),
    };
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL to be set");
    let pool = db::init_pool(&database_url).expect("Failed to create database connection pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(web::JsonConfig::default().limit(512))
            .wrap(middleware::Logger::default())
            .service(api::index)
            .service(api::create)
            .service(api::follow)
            .service(api::preview)
    })
    .bind(format!(
        "{}:{}",
        env::var("HOST").expect("HOST to be set"),
        env::var("PORT").expect("PORT to be set")
    ))?
    .start()
    .await
}
