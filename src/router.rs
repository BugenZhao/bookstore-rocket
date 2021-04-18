use crate::cors::CORS;
use crate::handler;

pub fn create_app() -> rocket::Rocket {
    rocket::ignite().attach(CORS).mount(
        "/books",
        routes![handler::books::get_all_books, handler::books::get_book],
    )
}
