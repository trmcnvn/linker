use serde::{Deserialize, Serialize};
use std::time::SystemTime;
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

#[derive(Deserialize, Serialize)]
pub struct CreateLink {
    pub external_url: String,
}
