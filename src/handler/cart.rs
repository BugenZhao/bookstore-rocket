use std::{collections::HashMap, sync::Mutex};

use rocket::{http::Status, State};
use rocket_contrib::json::Json;

use crate::auth::AuthedUser;

pub type Cart = HashMap<i32, i32>;
#[derive(Default)]
pub struct Carts(Mutex<HashMap<AuthedUser, Cart>>);

#[get("/")]
pub fn get_cart(carts: State<Carts>, user: AuthedUser) -> Result<Json<Cart>, Status> {
    let mut carts = carts.0.lock().unwrap();
    let cart = carts.entry(user).or_insert_with(Default::default);
    Ok(Json(cart.clone()))
}

#[put("/<book_id>")]
pub fn put_cart(book_id: i32, carts: State<Carts>, user: AuthedUser) -> Result<(), Status> {
    let mut carts = carts.0.lock().unwrap();
    let cart = carts.entry(user).or_insert_with(Default::default);
    *(cart.entry(book_id).or_insert(0)) += 1;
    Ok(())
}
