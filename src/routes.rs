use crate::handlers;
use crate::Database;
use warp::{Filter, Rejection, Reply};

pub fn routes(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    hello()
        .or(create(db.clone()))
        .or(follow(db.clone()))
        .or(preview(db))
}

/// GET /
fn hello() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end().and(warp::get()).and_then(handlers::hello)
}

/// POST /
fn create(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
        .and(warp::any().map(move || db.clone()))
        .and_then(handlers::create)
}

/// GET /{id}
fn follow(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::any().map(move || db.clone()))
        .and_then(handlers::follow)
}

/// GET /{id}/preview
fn preview(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path("preview"))
        .and(warp::path::end())
        .and(warp::any().map(move || db.clone()))
        .and_then(handlers::preview)
}
