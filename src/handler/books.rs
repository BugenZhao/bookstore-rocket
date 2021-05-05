use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::{
    auth::AuthedUser,
    db::{models::Book, schema, DbConn},
};
use diesel::prelude::*;
use std::collections::HashMap;

pub fn find_book_by_id(book_id: i32, conn: &DbConn) -> Option<Book> {
    use schema::books::dsl::*;
    books.find(book_id).get_result::<Book>(&*conn.0).ok()
}

#[get("/")]
pub fn get_all_books(conn: DbConn, _user: AuthedUser) -> Result<Json<HashMap<i32, Book>>, Status> {
    use schema::books::dsl::*;
    books
        .load::<Book>(&*conn)
        .map(|bs| bs.into_iter().map(|b| (b.id, b)).collect::<HashMap<_, _>>())
        .map(Json)
        .map_err(|_e| Status::InternalServerError)
}

#[get("/<book_id>")]
pub fn get_book(book_id: i32, conn: DbConn, _user: AuthedUser) -> Result<Json<Book>, Status> {
    find_book_by_id(book_id, &conn)
        .map(Json)
        .ok_or(Status::InternalServerError)
}
