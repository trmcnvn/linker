use crate::database::Database;
use std::env;
use warp::Filter;

mod database;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // logging
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "linker=info");
    }
    pretty_env_logger::init();

    // database
    let database_url = env::var("DATABASE_URL")?;
    let database = Database::new(&database_url)?;

    // routes
    let routes = routes::routes(database).with(warp::log("linker"));

    // server
    warp::serve(routes)
        .run((
            [0, 0, 0, 0],
            env::var("PORT").map_or(3030, |p| p.parse::<u16>().unwrap_or(3030)),
        ))
        .await;
    Ok(())
}
