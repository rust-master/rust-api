#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod schema;
mod models;

fn main() {
    println!("Hello API");
}