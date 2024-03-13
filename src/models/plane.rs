use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::database::Database;

#[derive(Serialize, Deserialize)]
pub struct Plane {
    id: u64,
    name: String,
    created_at: chrono::DateTime<Utc>
}

impl Database {

}