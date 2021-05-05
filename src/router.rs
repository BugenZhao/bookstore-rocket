use rocket_contrib::serve::StaticFiles;

use crate::handler::*;
use crate::{cors::CORS, db::init_pool};

pub fn create_app() -> rocket::Rocket {
    rocket::ignite()
        .manage(init_pool())
        .attach(CORS)
        .mount(
            "/static/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/books/", routes![books::get_all_books, books::get_book])
        .mount(
            "/users/",
            routes![users::login, users::check, users::logout, users::register],
        )
        .mount("/carousels/", routes![carousels::get_carousels])
}
