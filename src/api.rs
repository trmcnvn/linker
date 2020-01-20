use crate::db::Firestore;
use crate::models::{CreateLink, Link};
use rocket::{
    catch, get, post,
    response::{status, Redirect},
    Request, State,
};
use rocket_contrib::json::Json;
use std::time::SystemTime;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, World!"
}

#[post("/", format = "json", data = "<link>")]
pub fn create(
    link: Json<CreateLink>,
    db: State<Firestore>,
) -> Result<Json<Link>, status::BadRequest<&'static str>> {
    if link.0.external_url.is_empty() {
        return Err(status::BadRequest(Some(
            "You must provide the `external_url` value.",
        )));
    }

    let short_id = Link::generate_id(link.0.external_url.clone());
    let new_link = Link {
        external_url: link.0.external_url,
        short_id: short_id.clone(),
        created_at: SystemTime::now(),
    };

    match db.find("links", "short_id", short_id) {
        Ok(link) => Ok(Json(link)),
        Err(_) => match db.insert("links", &new_link) {
            Ok(_) => Ok(Json(new_link)),
            Err(_) => Err(status::BadRequest(Some("Sorry, the link wasn't created."))),
        },
    }
}

#[get("/<id>")]
pub fn follow(
    id: String,
    db: State<Firestore>,
) -> Result<Redirect, status::NotFound<&'static str>> {
    let link: Link = db
        .find("links", "short_id", id)
        .map_err(|_| status::NotFound("Sorry, that link doesn't exist."))?;
    Ok(Redirect::to(link.external_url))
}

#[get("/<id>/preview")]
pub fn preview(
    id: String,
    db: State<Firestore>,
) -> Result<Json<Link>, status::NotFound<&'static str>> {
    let link = db
        .find("links", "short_id", id)
        .map_err(|_| status::NotFound("Sorry, that link doesn't exist."))?;
    Ok(Json(link))
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry, {} is not a valid path.", req.uri())
}
