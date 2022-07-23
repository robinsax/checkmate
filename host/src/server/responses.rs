use std::fmt::{ Display };

use log::{ error };

use rocket::http::{ Status };
use rocket::serde::{ Serialize };
use rocket::serde::json::{ Json };
use rocket::request::{ Outcome, Request };
use rocket::response::{ Responder, Result as ResponderResult };

#[derive(Serialize, Clone,)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
pub enum ResponseCase<T: Serialize + Clone>  {
    Error(ErrorResponse),
    Success(T)
}

impl<'r, T: Serialize + Clone> Responder<'r, 'r> for ResponseCase<T> {
    fn respond_to(self, req: &Request) -> ResponderResult<'r> {
        match self {
            ResponseCase::Success(payload) => Json(payload.clone()).respond_to(req),
            ResponseCase::Error(err) => match Status::from_code(err.code) {
                Some(status) => {
                    let mut resp = Json(err.clone()).respond_to(req)?;
                    resp.set_status(status);
                    return Ok(resp);
                },
                _ => Err(Status::InternalServerError)
            }
        }
    }
}

pub fn error_response<T: Serialize + Clone, M: ToString + Display>(status: u16, message: M) -> ResponseCase<T> {
    error!("{}: {} (response)", status, message);

    ResponseCase::Error(ErrorResponse{
        code: status,
        message: message.to_string()
    })
}

pub fn error_outcome<S, M: ToString + Display>(status: u16, message: M) -> Outcome<S, M> {
    error!("{}: {} (outcome)", status, message);

    let status_enum = match Status::from_code(status) {
        Some(s) => s,
        _ => return Outcome::Failure((Status::InternalServerError, message)),
    };

    Outcome::Failure((status_enum, message))
}
