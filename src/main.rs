#![feature(proc_macro_hygiene, decl_macro)]

use firestore_db_and_auth::Credentials;
use rocket::{
    catchers,
    config::{Config, Environment},
    routes,
};

mod api;
mod db;
mod models;

fn main() {
    // database
    let credentials = Credentials::new(
        include_str!("../auth/gcloud-credentials.json"),
        &[
            include_str!("../auth/securetoken.jwk"),
            include_str!("../auth/service-account.jwk"),
        ],
    )
    .expect("Read Google credentials file");
    let db = db::Firestore::new(credentials);

    // web config
    let config = Config::build(Environment::active().expect("ROCKET_ENV to be valid"))
        .address("0.0.0.0")
        .port(
            std::env::var("PORT")
                .unwrap_or_else(|_| String::from("8080"))
                .parse()
                .unwrap(),
        )
        .keep_alive(0)
        .workers(4)
        .finalize()
        .expect("Config to be valid");

    // web
    rocket::custom(config)
        .manage(db)
        .mount(
            "/",
            routes![api::hello, api::create, api::follow, api::preview],
        )
        .register(catchers![api::not_found])
        .launch();
}
