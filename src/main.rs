#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod auth;
mod cors;
mod db;
mod handler;
mod router;

fn main() {
    dotenv::dotenv().unwrap();
    router::create_app().launch();
}
