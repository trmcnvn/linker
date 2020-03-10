use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct Link {
    pub external_url: String,
    pub short_id: String,
    pub created_at: SystemTime,
}

impl Link {
    pub fn generate_id(url: String) -> String {
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, url.as_bytes());
        let hash = seahash::hash(uuid.as_bytes());
        format!("{:x}", hash)
    }
}

impl From<Row> for Link {
    fn from(row: Row) -> Self {
        Self {
            external_url: row.get("external_url"),
            short_id: row.get("short_id"),
            created_at: row.get("created_at"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateLink {
    pub external_url: String,
}
