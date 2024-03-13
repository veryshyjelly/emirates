mod database;
mod models;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
}
