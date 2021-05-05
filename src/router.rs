use rocket_contrib::serve::StaticFiles;

use crate::handler::{cart::Carts, *};
use crate::{cors::CORS, db::init_pool};

pub fn create_app() -> rocket::Rocket {
    rocket::ignite()
        .manage(init_pool())
        .manage(Carts::default())
        .attach(CORS)
        .mount(
            "/resources/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/resources")),
        )
        .mount("/books/", routes![books::get_all_books, books::get_book])
        .mount(
            "/users/",
            routes![users::login, users::check, users::logout, users::register],
        )
        .mount("/carousels/", routes![carousels::get_carousels])
        .mount("/cart/", routes![cart::get_cart, cart::put_cart])
}
