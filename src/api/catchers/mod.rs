use crate::utils::errors::{Error, ErrorCode};
use rocket::Rocket;

#[catch(422)]
fn unprocessable_entity() -> Error {
    Error::custom(
        ErrorCode::InvalidData,
        "the given object couldn't be processed".to_string(),
    )
}

#[catch(404)]
fn not_found() -> Error {
    Error::custom(
        ErrorCode::ResourceNotFound,
        "the given route did not match any existing routes".to_string(),
    )
}

#[catch(500)]
fn internal_server_error() -> Error {
    Error::custom(
        ErrorCode::Unknown,
        "an internal server error occured while processing the request".to_string(),
    )
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.register(catchers![
        unprocessable_entity,
        not_found,
        internal_server_error
    ])
}
