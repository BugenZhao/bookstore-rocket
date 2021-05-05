use std::collections::HashMap;

use rocket::http::{Cookie, Cookies, Status};
use rocket_contrib::json::Json;

use crate::{
    auth::{AdminUser, AuthedUser},
    db::{models::UserAuth, schema, DbConn},
};
use diesel::prelude::*;

#[derive(Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

#[post("/login", data = "<info>")]
pub fn login(info: Json<LoginInfo>, conn: DbConn, mut cookies: Cookies) -> Result<(), Status> {
    use schema::user_auths::dsl::*;
    let user_auth = user_auths
        .filter(username.eq(&info.username))
        .filter(password.eq(&info.password))
        .get_result::<UserAuth>(&*conn)
        .ok();

    match user_auth {
        Some(user_auth) => {
            cookies.add_private(Cookie::new("user_id", user_auth.user_id.to_string()));
            Ok(())
        }
        None => Err(Status::Unauthorized), // TODO: report error
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Result<(), Status> {
    cookies.remove_private(Cookie::named("user_id"));
    Ok(())
}

#[get("/check")]
pub fn check(user: AuthedUser) -> Result<Json<AuthedUser>, Status> {
    Ok(Json(user))
}

#[get("/")]
pub fn get_all_users(
    conn: DbConn,
    _admin: AdminUser,
) -> Result<Json<HashMap<i32, AuthedUser>>, Status> {
    use schema::user_auths::dsl::*;
    user_auths
        .load::<UserAuth>(&*conn)
        .map(|uas| {
            uas.into_iter()
                .map(|ua| (ua.user_id, AuthedUser::from(&ua)))
                .collect::<HashMap<_, _>>()
        })
        .map(Json)
        .map_err(|_e| Status::InternalServerError)
}

#[derive(Deserialize)]
pub struct RegisterInfo {
    username: String,
    password: String,
}

#[post("/register", data = "<info>")]
pub fn register(
    info: Json<RegisterInfo>,
    conn: DbConn,
    mut cookies: Cookies,
) -> Result<(), Status> {
    use schema::user_auths::dsl::*;
    let already_exists = user_auths
        .filter(username.eq(&info.username))
        .get_result::<UserAuth>(&*conn)
        .is_ok();

    if already_exists {
        return Err(Status::BadRequest);
    }

    // TODO: real register

    let guest = user_auths
        .filter(username.eq("guest"))
        .get_result::<UserAuth>(&*conn)
        .ok();

    match guest {
        Some(user_auth) => {
            cookies.add_private(Cookie::new("user_id", user_auth.user_id.to_string()));
            Ok(())
        }
        None => Err(Status::Unauthorized), // TODO: report error
    }
}
