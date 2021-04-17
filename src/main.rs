#[macro_use]
extern crate diesel;

use db::{establish_connection, models::Book, schema};
use diesel::prelude::*;

mod db;

fn get_programming_books() -> Vec<Book> {
    use schema::books::dsl::*;
    let conn = establish_connection();
    books
        .order(price.desc())
        .filter(type_.eq_all("编程"))
        .load::<Book>(&conn)
        .unwrap()
}

fn main() {
    println!("{:#?}", get_programming_books());
}
