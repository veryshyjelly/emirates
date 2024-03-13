use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use crate::database::Database;

#[derive(Serialize, Deserialize)]
pub struct Seats {
    id: u64,
    plane: u64,
    class: u64,
    count: u64,
    fare: f32,
    created_at: DateTime<Utc>
}

impl Database {

}