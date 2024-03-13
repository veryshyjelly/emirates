use serde::{Deserialize, Serialize};
use crate::database::Database;

#[derive(Serialize, Deserialize)]
pub struct City {
    id: u64,
    name: String
}

impl Database {

}