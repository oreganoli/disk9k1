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
    let mut conn = app.pool.get()?;
    Ok(app.user.read(id, &mut conn)?.map(Json))
}

#[get("/users")]
pub fn get_all(app: AppState) -> AppResult<Json<Vec<User>>> {
    let app = app.read();
    let mut conn = app.pool.get()?;
    Ok(Json(app.user.read_all(&mut conn)?))
}

#[post("/users", data = "<new>")]
pub fn post(app: AppState, new: Json<NewUser>) -> AppResult<()> {
    let app = app.write();
    let mut conn = app.pool.get()?;
    app.user.create(new.into_inner(), &mut conn)
}
