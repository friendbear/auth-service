use super::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from_details<S: Into<String>, T: Into<String>>(email: S, hash: T) -> User {
        User {
            email: email.into(),
            hash: hash.into(),
            created_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: String,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl<T> From<T> for Invitation
where
    T: Into<String>,
    {
        fn from(email: T) ->Self {
            Invitation {
                id: Uuid::new_v4().to_string(),
                email: email.into(),
                expires_at: chrono::Utc::now().naive_local() + chrono::Duration::hours(24),
            }
        }
    }

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            email: user.email,
        }
    }
}