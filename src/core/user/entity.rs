use crate::schema::users;
use crate::utils::errors::{Error, ErrorCode};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum AccessLevel {
    Read,
    Write,
    Internal,
    Admin,
}

impl AccessLevel {
    pub fn from(s: String) -> Option<Self> {
        match s.as_str() {
            "read" => Some(AccessLevel::Read),
            "write" => Some(AccessLevel::Write),
            "internal" => Some(AccessLevel::Internal),
            "admin" => Some(AccessLevel::Admin),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AccessLevel::Read => "read".to_string(),
            AccessLevel::Write => "write".to_string(),
            AccessLevel::Internal => "internal".to_string(),
            AccessLevel::Admin => "admin".to_string(),
        }
    }

    pub fn is_readable(&self) -> bool {
        true
    }

    pub fn is_writable(&self) -> bool {
        self.clone() == AccessLevel::Write
            || self.clone() == AccessLevel::Internal
            || self.clone() == AccessLevel::Admin
    }

    pub fn is_internal(&self) -> bool {
        self.clone() == AccessLevel::Internal || self.clone() == AccessLevel::Admin
    }

    pub fn is_admin(&self) -> bool {
        self.clone() == AccessLevel::Admin
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub access_level: String,
}

impl User {
    pub fn validate(&self) -> Result<(), Error> {
        if AccessLevel::from(self.access_level.clone()).is_none() {
            return Err(Error::new(ErrorCode::InvalidData));
        }

        if self.id.len() == 0
            || self.name.len() == 0
            || self.password.len() == 0
            || self.access_level.len() == 0
        {
            Err(Error::new(ErrorCode::InvalidData))
        } else {
            Ok(())
        }
    }

    pub fn soft_validate(&self) -> Result<(), Error> {
        if AccessLevel::from(self.access_level.clone()).is_none() {
            return Err(Error::new(ErrorCode::InvalidData));
        }

        if self.id.len() == 0 || self.name.len() == 0 || self.access_level.len() == 0 {
            Err(Error::new(ErrorCode::InvalidData))
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AbstractUser {
    pub id: String,
    pub name: String,
    pub access_level: String,
}

impl AbstractUser {
    pub fn from(user: &User) -> Self {
        AbstractUser {
            id: user.id.clone(),
            name: user.name.clone(),
            access_level: user.access_level.clone(),
        }
    }
}
