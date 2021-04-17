use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::db::{establish_connection, models::Book, schema};
use diesel::prelude::*;

#[get("/")]
pub fn get_all_books() -> Result<Json<Vec<Book>>, Status> {
    use schema::books::dsl::*;
    let conn = establish_connection();
    books
        .load::<Book>(&conn)
        .map(Json)
        .map_err(|_e| Status::InternalServerError)
}

#[get("/<book_id>")]
pub fn get_book(book_id: i32) -> Result<Json<Book>, Status> {
    use schema::books::dsl::*;
    let conn = establish_connection();
    books
        .find(book_id)
        .get_result::<Book>(&conn)
        .map(Json)
        .map_err(|_e| Status::InternalServerError)
}
