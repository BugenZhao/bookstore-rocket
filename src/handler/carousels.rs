use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::auth::AuthedUser;

#[get("/")]
pub fn get_carousels(_user: AuthedUser) -> Result<Json<Vec<String>>, Status> {
    Ok(Json(
        (1..=4).map(|i| format!("/static/book{}.jpg", i)).collect(),
    ))
}
