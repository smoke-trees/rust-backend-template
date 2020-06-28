use super::entity::User;
use crate::utils::errors::Error;

pub trait Repository {
    fn add(&self, user: User) -> Result<(), Error>;
    fn remove(&self, user_id: String) -> Result<(), Error>;
    fn update(&self, user: User) -> Result<(), Error>;
    fn find(&self, user_id: String) -> Result<User, Error>;
    fn get_paged(&self, skip: i64, limit: i64) -> Result<Vec<User>, Error>;
}
