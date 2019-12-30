use crate::prelude::*;

pub mod repo;

#[derive(Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
    is_admin: bool,
}

#[derive(Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
    password: String,
    pass_con: String,
    #[serde(skip_deserializing)]
    #[serde(default = "default_privilege")]
    is_admin: bool,
}

fn default_privilege() -> bool {
    false
}

pub enum UserError {
    NameInvalid,
    NameTaken,
    EmailInvalid,
    EmailTaken,
    PasswordInvalid,
    PasswordsNotMatching,
}

#[get("/users/<id>")]
pub fn get(app: AppState, id: i32) -> AppResult<Option<Json<User>>> {
    let app = app.read();
    Ok(app.user.read(id, &mut app.pool.get()?)?.map(Json))
}
