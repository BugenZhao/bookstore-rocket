use diesel::{QueryDsl, RunQueryDsl};
use rocket::{http::Status, request::FromRequest, Outcome};

use crate::db::DbConn;
use crate::db::{models::UserAuth, schema};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthedUser {
    pub user_id: i32,
    pub username: String,
    pub user_type: i32,
}

impl From<&UserAuth> for AuthedUser {
    fn from(auth: &UserAuth) -> Self {
        Self {
            user_id: auth.user_id,
            username: auth.username.to_owned(),
            user_type: auth.user_type.to_owned(),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthedUser {
    type Error = ();

    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let user_auth = request.local_cache(|| {
            let conn = request.guard::<DbConn>().succeeded()?;
            let cookie = request.cookies().get_private("user_id")?;
            let target_user_id = cookie.value().parse::<i32>().ok()?;

            use schema::user_auths::dsl::*;
            user_auths
                .find(target_user_id)
                .get_result::<UserAuth>(&*conn)
                .ok()
        });

        match user_auth {
            Some(auth) => Outcome::Success(Self::from(auth)),
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}

pub struct AdminUser;

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = ();

    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let admin_user = request
            .guard::<AuthedUser>()
            .succeeded()
            .and_then(|authed_user| {
                if authed_user.user_type == 0 {
                    Some(Self)
                } else {
                    None
                }
            });

        match admin_user {
            Some(a) => Outcome::Success(a),
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
