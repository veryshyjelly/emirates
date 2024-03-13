use serde::{Deserialize, Serialize};
use crate::database::Database;

#[derive(Serialize, Deserialize)]
pub struct Customer {
    id: u64,
    name: String,
    phone: u64,
    email: String
}

impl Database {

}