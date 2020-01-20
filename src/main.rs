use actix_web::{middleware, web, App, HttpServer};
use firestore_db_and_auth::Credentials;
use std::env;

mod api;
mod db;
mod models;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    // logging
    match env::var("RUST_LOG") {
        Ok(_) => {}
        Err(_) => env::set_var("RUST_LOG", "main,actix_web"),
    };
    env_logger::init();

    // database
    let credentials = Credentials::new(
        include_str!("../auth/gcloud-credentials.json"),
        &[
            include_str!("../auth/securetoken.jwk"),
            include_str!("../auth/service-account.jwk"),
        ],
    )?;
    let db = db::Firestore::new(credentials);

    // web
    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .data(web::JsonConfig::default().limit(512))
            .wrap(middleware::Logger::default())
            .service(api::hello)
            .service(api::create)
            .service(api::follow)
            .service(api::preview)
    })
    .bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or_else(|_| String::from("8080"))
    ))?
    .workers(1)
    .run()
    .await
    .map_err(|err| anyhow::anyhow!(err))
}
