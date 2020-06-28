pub mod user;

use rocket::Rocket;

pub fn fuel(mut rocket: Rocket) -> Rocket {
    rocket = user::fuel(rocket);
    rocket
}
