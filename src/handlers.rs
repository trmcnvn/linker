use crate::models::{CreateLink, Link};
use crate::Database;
use std::convert::Infallible;
use std::time::SystemTime;
use warp::{http::StatusCode, Reply};

/// GET /
pub async fn hello() -> anyhow::Result<impl Reply, Infallible> {
    Ok("Hello, World\r\n".into_response())
}

/// POST /
pub async fn create(
    link: CreateLink,
    db: Database,
) -> anyhow::Result<Box<dyn warp::Reply>, Infallible> {
    if link.external_url.is_empty() {
        return Ok(Box::new(StatusCode::BAD_REQUEST));
    }

    let short_id = Link::generate_id(link.external_url.clone());
    let new_link = Link {
        external_url: link.external_url.clone(),
        short_id: short_id.clone(),
        created_at: SystemTime::now(),
    };

    match db.find_or_insert(&new_link).await {
        Ok(link) => Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&link),
            StatusCode::CREATED,
        ))),
        Err(_) => Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

/// GET /{id}
pub async fn follow(id: String, db: Database) -> anyhow::Result<Box<dyn Reply>, Infallible> {
    match db.find(&id).await {
        Ok(link) => Ok(Box::new(warp::reply::with_header(
            warp::reply(),
            "Location",
            link.external_url,
        ))),
        Err(_) => Ok(Box::new(StatusCode::NOT_FOUND)),
    }
}

/// GET /{id}/preview
pub async fn preview(id: String, db: Database) -> anyhow::Result<Box<dyn Reply>, Infallible> {
    match db.find(&id).await {
        Ok(link) => Ok(Box::new(warp::reply::json(&link))),
        Err(_) => Ok(Box::new(StatusCode::NOT_FOUND)),
    }
}
