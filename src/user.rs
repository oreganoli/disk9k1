use schema::users;

use crate::prelude::*;

/// The publicly-visible information about users.
#[derive(Serialize)]
pub struct UserInfo<'a> {
    pub id: i32,
    pub name: &'a str,
    pub joined: NaiveDateTime,
    pub is_admin: bool,
}

#[derive(Queryable, Insertable)]
struct User {
    id: Option<i32>,
    name: String,
    /// Self-explanatory. Hashed.
    password: String,
    /// The quick-upload token, hashed.
    quick_token: String,
    joined: Option<NaiveDateTime>,
    is_admin: bool,
}

impl User {
    /// A constructor lacking the `id` and `joined` fields, which the DB should supply by itself.
    pub fn new(name: String, password: String, quick_token: String, is_admin: bool) -> Self {
        User {
            id: None,
            name,
            password,
            quick_token,
            joined: None,
            is_admin,
        }
    }
    fn to_info(&self) -> UserInfo {
        UserInfo {
            id: self.id.expect("Could not get an ID out of the User struct"),
            name: &self.name,
            joined: self
                .joined
                .expect("Could not get a joining date out of the User struct"),
            is_admin: self.is_admin,
        }
    }
}
