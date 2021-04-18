use crate::cors::CORS;
use crate::handler;

pub fn create_app() -> rocket::Rocket {
    rocket::ignite()
        .attach(CORS)
        .mount("/books", routes![handler::get_all_books, handler::get_book])
}
