use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::database::Database;

#[derive(Serialize, Deserialize)]
pub struct Distance {
    id: u64,
    city1: u64,
    city2: u64,
    distance: f32,
    created_at: DateTime<Utc>
}

impl Database {

}