use crate::db;
use actix_web::{error, get, http, post, web, Error, HttpResponse, Result};
use serde_derive::Deserialize;

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, World!\r\n"
}

#[derive(Deserialize)]
pub struct CreateLink {
    external_url: String,
}

#[post("/")]
pub async fn create(
    params: web::Json<CreateLink>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    if params.external_url.is_empty() {
        Err(error::ErrorBadRequest("You must provide an external_url."))
    } else {
        match web::block(move || db::create_link(params.into_inner().external_url, &pool)).await {
            Ok(link) => Ok(HttpResponse::Ok().json(link)),
            Err(err) => Err(error::ErrorInternalServerError(err)),
        }
    }
}

#[get("/{id}")]
pub async fn follow(
    params: web::Path<String>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    match web::block(move || db::find_by_short(params.into_inner(), &pool)).await {
        Ok(link) => Ok(HttpResponse::Found()
            .header(http::header::LOCATION, link.external_url)
            .finish()),
        Err(err) => Err(error::ErrorNotFound(err)),
    }
}

#[get("/{id}/preview")]
pub async fn preview(
    params: web::Path<String>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    match web::block(move || db::find_by_short(params.into_inner(), &pool)).await {
        Ok(link) => Ok(HttpResponse::Ok().json(link)),
        Err(err) => Err(error::ErrorNotFound(err)),
    }
}
