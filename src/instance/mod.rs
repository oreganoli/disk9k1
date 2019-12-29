use crate::prelude::*;

pub mod repo;

#[derive(Debug, Serialize)]
pub struct Instance {
    pub name: String,
    pub description: String,
    pub size_limit: usize,
    pub version: String,
}

#[get("/instance")]
pub fn get(app: AppState) -> AppResult<Json<Instance>> {
    let app = app.read();
    let inst = app.instance.read(&mut app.pool.get()?)?;
    Ok(Json(inst))
}
