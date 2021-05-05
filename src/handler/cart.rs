use std::{collections::HashMap, sync::Mutex};

use rocket::{http::Status, State};
use rocket_contrib::json::Json;

use crate::{
    auth::AuthedUser,
    db::{models::Book, DbConn},
};

use super::books::find_book_by_id;

type Cart = HashMap<i32, i32>;
#[derive(Default)]
pub struct Carts(Mutex<HashMap<AuthedUser, Cart>>);

#[derive(Serialize)]
struct BookWithCount {
    book: Book,
    count: i32,
}
#[derive(Serialize)]
pub struct CartResponse {
    books: Vec<BookWithCount>,
    discount: f64,
    total: f64,
}

#[get("/")]
pub fn get_cart(
    carts: State<Carts>,
    conn: DbConn,
    user: AuthedUser,
) -> Result<Json<CartResponse>, Status> {
    let mut carts = carts.0.lock().unwrap();
    let cart = carts.entry(user).or_insert_with(Default::default);
    let books = cart
        .iter()
        .map(|(&book_id, &count)| {
            let book = find_book_by_id(book_id, &conn).unwrap();
            BookWithCount { book, count }
        })
        .collect::<Vec<_>>();

    let total: f64 = books.iter().map(|b| b.book.price * (b.count as f64)).sum();
    let discount = (total * 0.3).min(100.0);

    Ok(Json(CartResponse {
        books,
        discount,
        total: total - discount,
    }))
}

#[put("/<book_id>")]
pub fn put_a_book(book_id: i32, carts: State<Carts>, user: AuthedUser) -> Result<(), Status> {
    let mut carts = carts.0.lock().unwrap();
    let cart = carts.entry(user).or_insert_with(Default::default);
    *(cart.entry(book_id).or_insert(0)) += 1;
    Ok(())
}

#[delete("/<book_id>")]
pub fn delete_books(book_id: i32, carts: State<Carts>, user: AuthedUser) -> Result<(), Status> {
    let mut carts = carts.0.lock().unwrap();
    let cart = carts.entry(user).or_insert_with(Default::default);
    cart.remove(&book_id);
    Ok(())
}

#[delete("/")]
pub fn empty_cart(carts: State<Carts>, user: AuthedUser) -> Result<(), Status> {
    let mut carts = carts.0.lock().unwrap();
    carts.remove(&user);
    Ok(())
}
