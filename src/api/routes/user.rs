use crate::api::misc::structs::{Credentials, EntityStringId, Page, PageData};
use crate::core::user::entity::{AbstractUser, AccessLevel, User};
use crate::core::user::service::Service;
use crate::utils::errors::{Error, ErrorCode, Success};

use crate::api::guards::auth::{ClaimResult, Claims};
use rocket::Rocket;
use rocket_contrib::json::Json;
use serde_json::Value;

#[post("/setup")]
fn setup(service: Service) -> Result<Success, Error> {
    let user = User {
        id: "admin".to_string(),
        name: "admin".to_string(),
        password: "admin".to_string(),
        access_level: "admin".to_string(),
    };

    service.add(user)?;
    Ok(Success::new("added admin successfully"))
}

#[post("/add", format = "json", data = "<user>")]
fn add(user: Json<User>, service: Service, claims: ClaimResult) -> Result<Success, Error> {
    let claims = claims.inner()?;

    if !claims.access_level.is_admin() {
        return Err(Error::new(ErrorCode::NotAuthorized));
    }

    user.validate()?;
    service.add(user.into_inner())?;

    Ok(Success::new("added user successfully"))
}

#[post("/remove", format = "json", data = "<user_id>")]
fn remove(
    user_id: Json<EntityStringId>,
    service: Service,
    claims: ClaimResult,
) -> Result<Success, Error> {
    let claims = claims.inner()?;

    if !claims.access_level.is_admin() {
        return Err(Error::new(ErrorCode::NotAuthorized));
    }

    service.remove(user_id.into_inner().id)?;
    Ok(Success::new("removed user successfully"))
}

#[post("/update", format = "json", data = "<user>")]
fn update(user: Json<User>, service: Service, claims: ClaimResult) -> Result<Success, Error> {
    let claims = claims.inner()?;

    if !claims.access_level.is_admin() {
        return Err(Error::new(ErrorCode::NotAuthorized));
    }

    let user = user.into_inner();
    user.soft_validate()?;

    if service.find(user.id.clone())?.access_level == "admin" {
        if user.access_level != "admin" {
            return Err(Error::new(ErrorCode::NotAuthorized));
        }
    }

    service.update(user)?;

    Ok(Success::new("updated user successfully"))
}

#[post("/retrieve", format = "json", data = "<page>")]
fn retrieve(
    page: Json<Page>,
    service: Service,
    claims: ClaimResult,
) -> Result<Json<PageData<AbstractUser>>, Error> {
    let claims = claims.inner()?;

    if !claims.access_level.is_admin() {
        return Err(Error::new(ErrorCode::NotAuthorized));
    };

    let abstract_users = service.retrieve(page.skip, page.limit)?;
    Ok(Json(PageData {
        data: abstract_users,
        msg: "retrieved users successfully".to_string(),
    }))
}

#[post("/login", format = "json", data = "<creds>")]
fn login(creds: Json<Credentials>, service: Service) -> Result<Json<Value>, Error> {
    service.login(creds.id.clone(), creds.password.clone())?;

    let user = service.find(creds.id.clone())?;
    let level = AccessLevel::from(user.access_level);

    if level.is_none() {
        return Err(Error::custom(
            ErrorCode::CorruptResource,
            "undefined access level found".to_string(),
        ));
    }

    let claims = Claims::new(user.id, level.unwrap());
    let token_data = claims.jwt()?;

    Ok(Json(json! ({
        "data": token_data,
        "msg": "generated token successfully".to_string(),
    })))
}

#[post("/get")]
fn get(claims: ClaimResult) -> Result<Json<Value>, Error> {
    let claims = claims.inner()?;

    return Ok(Json(json!({
        "id": claims.id,
        "access_level": claims.access_level
    })))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/api/users",
        routes![add, remove, update, retrieve, login, setup, get],
    )
}
