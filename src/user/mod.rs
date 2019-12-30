use crate::prelude::*;

pub mod repo;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    password: String,
    pub(crate) is_admin: bool,
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
    AdminDeletion,
}

#[get("/me")]
pub fn me(app: AppState, mut cookies: Cookies) -> AppResult<Json<User>> {
    let app = app.read();
    let mut conn = app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, &mut conn)?;
    Ok(Json(user))
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

#[derive(Deserialize)]
pub struct DelRequest {
    id: i32,
}

#[derive(Deserialize)]
pub struct PassRequest {
    new: String,
    con: String,
}

#[derive(Deserialize)]
pub struct NameRequest {
    new: String,
}

#[put("/change_password", data = "<pass_req>")]
pub fn put_password(
    app: AppState,
    pass_req: Json<PassRequest>,
    mut cookies: Cookies,
) -> AppResult<()> {
    let req = pass_req.into_inner();
    if req.new != req.con {
        return Err(UserError::PasswordsNotMatching.into());
    }
    let app = app.write();
    let mut conn = app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, &mut conn)?;
    app.user.update_password(user.id, &req.new, &mut conn)?;
    cookies.add_private(Cookie::new("password", req.new));
    Ok(())
}

#[put("/change_username", data = "<name_req>")]
pub fn put_username(
    app: AppState,
    name_req: Json<NameRequest>,
    mut cookies: Cookies,
) -> AppResult<()> {
    let app = app.write();
    let mut conn = app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, &mut conn)?;
    let req = name_req.into_inner();
    app.user.update_username(user.id, &req.new, &mut conn)?;
    cookies.add_private(Cookie::new("username", req.new));
    Ok(())
}

#[delete("/users", data = "<del_req>")]
pub fn delete(app: AppState, del_req: Json<DelRequest>, mut cookies: Cookies) -> AppResult<()> {
    let app = app.write();
    let mut conn = app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, &mut conn)?;
    if user.id == del_req.id {
        if user.is_admin {
            Err(UserError::AdminDeletion.into())
        } else {
            app.user.delete(del_req.id, &mut conn)?;
            cookies.remove_private(Cookie::named("username"));
            cookies.remove_private(Cookie::named("password"));
            Ok(())
        }
    } else if user.is_admin {
        app.user.delete(del_req.id, &mut conn)?;
        Ok(())
    } else {
        Err(AuthError::NotAllowed.into())
    }
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
