#![feature(proc_macro_hygiene, decl_macro)]

use firestore_db_and_auth::Credentials;
use rocket::{catchers, routes};

mod api;
mod db;
mod models;

fn main() {
    // database
    let credentials =
        Credentials::from_file("gcloud-credentials.json").expect("Read Google credentials file");
    let db = db::Firestore::new(credentials);

    // web
    rocket::ignite()
        .manage(db)
        .mount(
            "/",
            routes![api::hello, api::create, api::follow, api::preview],
        )
        .register(catchers![api::not_found])
        .launch();
}
