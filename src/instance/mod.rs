use crate::prelude::*;

pub mod repo;

fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Instance {
    pub name: String,
    pub description: String,
    pub size_limit: usize,
    #[serde(default = "version")]
    #[serde(skip_deserializing)]
    pub version: String,
}

#[get("/instance")]
pub fn get(app: AppState) -> AppResult<Json<Instance>> {
    let app = app.read();
    let inst = app.instance.read(&mut app.pool.get()?)?;
    Ok(Json(inst))
}

#[put("/instance", data = "<instance>")]
pub fn put(app: AppState, instance: Json<Instance>, mut cookies: Cookies) -> AppResult<()> {
    let app = app.write();
    let mut conn = app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, &mut conn)?;
    if !user.is_admin {
        Err(AuthError::NotAllowed.into())
    } else {
        app.instance.update(instance.into_inner(), &mut conn)
    }
}
