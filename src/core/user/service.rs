use super::diesel::DieselRepo;
use super::entity::User;
use super::repository::Repository;
use bcrypt;

use crate::core::user::entity::AbstractUser;
use crate::utils::db::Pool;
use crate::utils::errors::{Error, ErrorCode};
use rocket::request::{FromRequest, Outcome};
use rocket::{http::Status, Request, State};

pub struct Service {
    repo: Box<dyn Repository + Send + Sync>,
}

impl Service {
    pub fn new(repo: Box<dyn Repository + Send + Sync>) -> Self {
        Service { repo }
    }

    pub fn add(&self, mut user: User) -> Result<(), Error> {
        if self.repo.find(user.id.clone()).is_ok() {
            return Err(Error::new(ErrorCode::ResourceAlreadyExists));
        }

        let hash = bcrypt::hash(user.password.clone(), bcrypt::DEFAULT_COST);

        if hash.is_err() {
            return Err(Error::custom(
                ErrorCode::Unknown,
                "bcrypt hash generation failed".to_string(),
            ));
        }

        user.password = hash.unwrap();
        self.repo.add(user)
    }

    pub fn remove(&self, user_id: String) -> Result<(), Error> {
        self.repo.remove(user_id)
    }

    pub fn update(&self, mut user: User) -> Result<(), Error> {
        if self.repo.find(user.id.clone()).is_err() {
            return Err(Error::new(ErrorCode::ResourceNotFound));
        }

        if user.password.len() > 0 {
            let hash = bcrypt::hash(user.password.clone(), bcrypt::DEFAULT_COST);
            if hash.is_err() {
                return Err(Error::custom(
                    ErrorCode::Unknown,
                    "bcrypt hash generation failed".to_string(),
                ));
            }

            user.password = hash.unwrap();
        }

        self.repo.update(user)
    }

    pub fn login(&self, user_id: String, passwd: String) -> Result<(), Error> {
        let user = self.repo.find(user_id)?;
        let result = bcrypt::verify(passwd, user.password.as_str());

        if result.is_err() {
            Err(Error::custom(
                ErrorCode::InvalidCredentials,
                "bcrypt verification failed".to_string(),
            ))
        } else {
            if result.unwrap() {
                Ok(())
            } else {
                Err(Error::custom(
                    ErrorCode::InvalidCredentials,
                    "password mismatch".to_string(),
                ))
            }
        }
    }

    pub fn retrieve(&self, skip: i64, limit: i64) -> Result<Vec<AbstractUser>, Error> {
        let users = self.repo.get_paged(skip, limit)?;
        let mut abstract_users = Vec::<AbstractUser>::new();
        for user in &users {
            abstract_users.push(AbstractUser::from(user));
        }

        Ok(abstract_users)
    }

    pub fn find(&self, user_id: String) -> Result<User, Error> {
        self.repo.find(user_id)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Service {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pool_result = request.guard::<State<Pool>>();

        if let Outcome::Failure(_) = pool_result {
            return Outcome::Failure((
                Status::ServiceUnavailable,
                Error::custom(
                    ErrorCode::DatabaseError,
                    "couldn't fetch connection from pool".to_string(),
                ),
            ));
        }

        let pool = pool_result.unwrap();

        let service = Service::new(Box::new(DieselRepo::new(pool.to_owned())));
        Outcome::Success(service)
    }
}
