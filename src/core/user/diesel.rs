use diesel::prelude::*;

use super::entity::User;
use super::repository::Repository;

use crate::utils::db::{get_connection, Pool};
use crate::utils::errors::{Error, ErrorCode};

use crate::schema::users::dsl::*;

pub struct DieselRepo {
    pool: Pool,
}

impl DieselRepo {
    pub fn new(pool: Pool) -> Self {
        DieselRepo { pool }
    }
}

impl Repository for DieselRepo {
    fn add(&self, user: User) -> Result<(), Error> {
        let conn = get_connection(&self.pool)?;

        if users.find(user.id.clone()).first::<User>(&conn).is_ok() {
            return Err(Error::new(ErrorCode::ResourceAlreadyExists));
        }

        let result = diesel::insert_into(users).values(&user).execute(&conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::custom(ErrorCode::DatabaseError, e.to_string())),
        }
    }

    fn remove(&self, user_id: String) -> Result<(), Error> {
        let conn = get_connection(&self.pool)?;

        if users.find(user_id.clone()).first::<User>(&conn).is_err() {
            return Err(Error::new(ErrorCode::ResourceNotFound));
        }

        let result = diesel::delete(users.filter(id.eq(user_id))).execute(&conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::custom(ErrorCode::DatabaseError, e.to_string())),
        }
    }

    fn update(&self, user: User) -> Result<(), Error> {
        let conn = get_connection(&self.pool)?;

        if users.find(user.id.clone()).first::<User>(&conn).is_err() {
            return Err(Error::new(ErrorCode::ResourceNotFound));
        }

        let result = if user.password.len() > 0 {
            diesel::update(users.filter(id.eq(user.id.clone())))
                .set(&user)
                .execute(&conn)
        } else {
            diesel::update(users.filter(id.eq(user.id.clone())))
                .set((
                    id.eq(user.id),
                    name.eq(user.name),
                    access_level.eq(user.access_level),
                ))
                .execute(&conn)
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::custom(ErrorCode::DatabaseError, e.to_string())),
        }
    }

    fn find(&self, user_id: String) -> Result<User, Error> {
        let conn = get_connection(&self.pool)?;

        if let Ok(user) = users.find(user_id.clone()).first::<User>(&conn) {
            Ok(user)
        } else {
            Err(Error::new(ErrorCode::ResourceNotFound))
        }
    }

    fn get_paged(&self, skip: i64, limit: i64) -> Result<Vec<User>, Error> {
        let conn = get_connection(&self.pool)?;

        let result = users.offset(skip).limit(limit).load::<User>(&conn);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::custom(ErrorCode::DatabaseError, e.to_string())),
        }
    }
}
