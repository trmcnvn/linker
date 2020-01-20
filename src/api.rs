use crate::db::Firestore;
use crate::models::{CreateLink, Link};
use actix_web::{error, get, http, post, web, Error, HttpResponse};
use std::time::SystemTime;

#[get("/")]
pub async fn hello() -> &'static str {
    "Hello, World!\r\n"
}

#[post("/")]
pub async fn create(
    params: web::Json<CreateLink>,
    db: web::Data<Firestore>,
) -> Result<HttpResponse, Error> {
    if params.external_url.is_empty() {
        return Err(error::ErrorBadRequest(
            "You must provide an `external_url` value.",
        ));
    }

    let short_id = Link::generate_id(params.external_url.clone());
    let new_link = Link {
        external_url: params.external_url.clone(),
        short_id: short_id.clone(),
        created_at: SystemTime::now(),
    };

    match web::block(move || db.find_or_insert("links", "short_id", short_id, &new_link)).await {
        Ok(link) => Ok(HttpResponse::Ok().json(link)),
        Err(_) => Err(error::ErrorBadRequest("The link wasn't created.")),
    }
}

#[get("/{id}")]
pub async fn follow(
    params: web::Path<String>,
    db: web::Data<Firestore>,
) -> Result<HttpResponse, Error> {
    let link: Link = web::block(move || db.find("links", "short_id", params.into_inner()))
        .await
        .map_err(|_| error::ErrorNotFound("That link doesn't exist."))?;
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, link.external_url)
        .finish())
}

#[get("/{id}/preview")]
pub async fn preview(
    params: web::Path<String>,
    db: web::Data<Firestore>,
) -> Result<HttpResponse, Error> {
    let link: Link = web::block(move || db.find("links", "short_id", params.into_inner()))
        .await
        .map_err(|_| error::ErrorNotFound("That link doesn't exist."))?;
    Ok(HttpResponse::Ok().json(link))
}

// #[catch(404)]
// pub fn not_found(req: &Request) -> String {
//     format!("Sorry, {} is not a valid path.", req.uri())
// }
