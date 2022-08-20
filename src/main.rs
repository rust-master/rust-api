#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::models::Book;

mod schema;
mod models;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Shades of flowers"),
        author: String::from("Zaryab"),
        published: true,
    };

    if models::Book::insert(book, &conn) {
        println!("success");
        
    } else {
        println!("failed");
    }

    let booki = models::Book::all(&conn);
    for i in booki.iter() {
        println!("ID: {} , Title: {}, Author: {}", i.id, i.title, i.author);
    }
}
