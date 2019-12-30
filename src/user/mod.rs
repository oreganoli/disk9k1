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

#[post("/login", data = "<auth_req>")]
pub fn login(app: AppState, auth_req: Json<AuthRequest>, mut cookies: Cookies) -> AppResult<()> {
    let app = app.read();
    let mut conn = app.pool.get()?;
    let mut name_ck = Cookie::named("username");
    let mut pass_ck = Cookie::named("password");
    let user = app.user.read_name(&auth_req.username, &mut conn)?;
    if user.is_none() {
        cookies.remove_private(name_ck);
        cookies.remove_private(pass_ck);
        return Err(AuthError::InvalidCredentials.into());
    }
    if !bcrypt::verify(&auth_req.password, &user.unwrap().password)? {
        cookies.remove_private(name_ck);
        cookies.remove_private(pass_ck);
        return Err(AuthError::InvalidCredentials.into());
    }
    let req = auth_req.into_inner();
    name_ck.set_value(req.username);
    pass_ck.set_value(req.password);
    cookies.add_private(name_ck);
    cookies.add_private(pass_ck);
    Ok(())
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) {
    cookies.remove_private(Cookie::named("username"));
    cookies.remove_private(Cookie::named("password"));
}

#[derive(Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

pub enum AuthError {
    InvalidCredentials,
    NotAllowed,
}
