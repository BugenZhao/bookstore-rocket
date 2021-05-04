use rocket::{http::Status, request::FromRequest, Outcome};

pub struct User {
    pub name: String,
    pub is_admin: bool,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let user = request.cookies().get_private("user");

        match user {
            Some(cookie) => {
                // TODO: query the db
                let username = cookie.value();
                Outcome::Success(User {
                    name: username.to_owned(),
                    is_admin: ["admin", "bugen"].contains(&username),
                })
            }
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
