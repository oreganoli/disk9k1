use schema::users;

use crate::prelude::*;

mod repo;

/// The publicly-visible information about users.
#[derive(Serialize)]
pub struct UserInfo<'a> {
    pub id: i32,
    pub name: &'a str,
    pub joined: NaiveDateTime,
    pub is_admin: bool,
}

#[derive(Queryable, Clone)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    /// Self-explanatory. Hashed.
    password: String,
    /// The quick-upload token, hashed.
    quick_token: String,
    joined: NaiveDateTime,
    is_admin: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
/// An `Insertable` struct that lacks the `id` and `joined` fields of an actual `User`, which are assigned by the DB.
pub struct NewUser {
    name: String,
    email: String,
    password: String,
    quick_token: String,
    is_admin: bool,
}

impl NewUser {
    pub fn new(
        name: String,
        email: String,
        password: String,
        quick_token: String,
        is_admin: bool,
    ) -> Self {
        Self {
            name,
            email,
            password,
            quick_token,
            is_admin,
        }
    }
}

impl User {
    fn to_info(&self) -> UserInfo {
        UserInfo {
            id: self.id,
            name: &self.name,
            joined: self.joined,
            is_admin: self.is_admin,
        }
    }
}
