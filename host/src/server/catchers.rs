use rocket::{ catch, catchers };
use rocket::fairing::{ AdHoc as AdHocFairing };
use rocket::serde::json::{ Json };

use super::responses::{ ErrorResponse };

#[catch(404)]
fn not_found_catcher() -> Json<ErrorResponse> {
    Json(ErrorResponse{
        code: 404,
        message: "no endpoint".to_string()
    })
}

#[catch(401)]
fn unauthorized_catcher() -> Json<ErrorResponse> {
    Json(ErrorResponse{
        code: 401,
        message: "unauthorized".to_string()
    })
}

pub fn stage() -> AdHocFairing {
    AdHocFairing::on_ignite("catchers", |rocket| async {
        rocket.register("/", catchers![not_found_catcher, unauthorized_catcher])
    })
}
