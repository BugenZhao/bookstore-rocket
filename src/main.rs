#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod db;
mod handler;
mod router;
mod cors;

fn main() {
    dotenv::dotenv().unwrap();
    router::create_app().launch();
}
