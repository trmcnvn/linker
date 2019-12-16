use crate::model;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use std::ops::Deref;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get()
        .map_err(|_| "Couldn't get connection to the database pool.")
}

pub fn create_link(external_url: String, pool: &PgPool) -> Result<model::Link, &'static str> {
    match find_by_external(external_url.clone(), pool) {
        Ok(link) => Ok(link),
        Err(_) => {
            let new_link = model::NewLink::new(external_url);
            model::Link::insert(new_link, get_conn(pool)?.deref())
                .map_err(|_| "Failed to insert link to the database.")
        }
    }
}

pub fn find_by_short(id: String, pool: &PgPool) -> Result<model::Link, &'static str> {
    model::Link::find_by_short(id, get_conn(pool)?.deref()).map_err(|_| "Link doesn't exist.")
}

pub fn find_by_external(external: String, pool: &PgPool) -> Result<model::Link, &'static str> {
    model::Link::find_by_external(external, get_conn(pool)?.deref())
        .map_err(|_| "Link doesn't exist.")
}
