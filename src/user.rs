use schema::users;

use crate::prelude::*;

/// The publicly-visible information about users.
#[derive(Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
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
