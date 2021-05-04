use crate::handler;
use crate::{cors::CORS, db::init_pool};

pub fn create_app() -> rocket::Rocket {
    rocket::ignite().manage(init_pool()).attach(CORS).mount(
        "/books",
        routes![handler::books::get_all_books, handler::books::get_book],
    )
}
