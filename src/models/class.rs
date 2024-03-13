use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use crate::database::Database;

#[derive(Serialize, Deserialize)]
pub struct Class {
    id: u64,
    name: String,
    created_at: DateTime<Utc>
}

impl Database {

}