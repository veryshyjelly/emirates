use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::database::Database;

#[derive(Serialize, Deserialize)]
struct Reservation {
    id: u64,
    customer: u64,
    schedule: u64,
    fare: f32,
    created_at: DateTime<Utc>
}

impl Database {

}