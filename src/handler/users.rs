use rocket::http::{Cookie, Cookies, Status};
use rocket_contrib::json::Json;

use crate::{
    auth::User,
    db::{models::Book, schema, DbConn},
};
use diesel::prelude::*;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

#[post("/login", data = "<info>")]
pub fn login(info: Json<LoginInfo>, conn: DbConn, mut cookies: Cookies) -> Result<(), Status> {
    // TODO: query the db
    cookies.add_private(Cookie::new("user", info.username.clone()));
    Ok(())
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Result<(), Status> {
    cookies.remove_private(Cookie::named("user"));
    Ok(())
}

#[get("/check")]
pub fn check(user: User) -> Result<String, Status> {
    Ok(user.name)
}
