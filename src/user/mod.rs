use schema::users;

use crate::prelude::*;

pub mod auth;
pub mod delete;
pub mod info;
pub mod register;
pub mod repo;
//pub mod settings;

/// The publicly-visible information about users.
#[derive(Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub joined: NaiveDateTime,
    pub is_admin: bool,
}

#[derive(Queryable, Identifiable, Clone)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    /// Self-explanatory. Hashed.
    password: String,
    joined: NaiveDateTime,
    is_admin: bool,
}

impl User {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn joined(&self) -> NaiveDateTime {
        self.joined
    }
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
    pub fn to_info(&self) -> UserInfo {
        UserInfo {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            joined: self.joined,
            is_admin: self.is_admin,
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
/// An `Insertable` struct that lacks the `id` and `joined` fields of an actual `User`, which are assigned by the DB.
pub struct NewUser {
    name: String,
    email: String,
    password: String,
    is_admin: bool,
}

impl NewUser {
    pub fn new(name: String, email: String, password: String, is_admin: bool) -> Self {
        Self {
            name,
            email,
            password,
            is_admin,
        }
    }
    pub fn generate_admin() -> Self {
        use std::env::var;
        Self {
            name: var("ADMIN_USERNAME")
                .expect("The environment variable ADMIN_USERNAME must be set."),
            email: var("ADMIN_EMAIL").expect("The environment variable ADMIN_EMAIL must be set."),
            password: var("ADMIN_PASSWORD")
                .expect("The environment variable ADMIN_PASSWORD must be set."),
            is_admin: true,
        }
    }
}
