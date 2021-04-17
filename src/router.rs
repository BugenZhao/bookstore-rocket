use crate::handler;

pub fn create_app() -> rocket::Rocket {
    rocket::ignite().mount("/books", routes![handler::get_all_books, handler::get_book])
}
