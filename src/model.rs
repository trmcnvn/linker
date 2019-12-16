use crate::schema::links::{
    self as links_schema,
    dsl::{external_url, links, short_id},
};
use diesel::{self, pg::PgConnection, prelude::*};
use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "links_schema"]
pub struct NewLink {
    pub external_url: String,
    pub short_id: String,
}

impl NewLink {
    pub fn new(external: String) -> Self {
        Self {
            external_url: external.clone(),
            short_id: Self::generate_id(external),
        }
    }

    fn generate_id(url: String) -> String {
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, url.as_bytes());
        let hash = seahash::hash(uuid.as_bytes());
        format!("{:x}", hash)
    }
}

#[derive(Debug, Queryable, Serialize)]
pub struct Link {
    pub id: i32,
    pub external_url: String,
    pub short_id: String,
    pub created_at: SystemTime,
}

impl Link {
    pub fn insert(link: NewLink, conn: &PgConnection) -> QueryResult<Link> {
        diesel::insert_into(links_schema::table)
            .values(&link)
            .get_result(conn)
    }

    pub fn find_by_short(id: String, conn: &PgConnection) -> QueryResult<Link> {
        links.filter(short_id.eq(id)).first(conn)
    }

    pub fn find_by_external(external: String, conn: &PgConnection) -> QueryResult<Link> {
        links.filter(external_url.eq(external)).first(conn)
    }
}
