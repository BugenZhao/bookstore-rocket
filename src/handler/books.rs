use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::{
    auth::User,
    db::{models::Book, schema, DbConn},
};
use diesel::prelude::*;
use std::collections::HashMap;

#[get("/")]
pub fn get_all_books(conn: DbConn, _user: User) -> Result<Json<HashMap<i32, Book>>, Status> {
    use schema::books::dsl::*;
    books
        .load::<Book>(&*conn)
        .map(|bs| bs.into_iter().map(|b| (b.id, b)).collect::<HashMap<_, _>>())
        .map(Json)
        .map_err(|_e| Status::InternalServerError)
}

#[get("/<book_id>")]
pub fn get_book(book_id: i32, conn: DbConn, _user: User) -> Result<Json<Book>, Status> {
    use schema::books::dsl::*;
    books
        .find(book_id)
        .get_result::<Book>(&*conn)
        .map(Json)
        .map_err(|_e| Status::InternalServerError)
}
