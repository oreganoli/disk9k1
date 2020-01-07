use rocket::http::ContentType;
use rocket::Data;

use crate::prelude::*;

pub mod multi;

pub struct NewFile {
    filename: String,
    public: bool,
    directory: Option<i32>,
    data: Vec<u8>,
}

pub enum FileError {
    NamingConflict,
    NoFileName,
    TooBig,
    ImproperForm,
}

#[derive(Serialize)]
pub struct File {
    pub id: i32,
    pub filename: String,
    pub hash: i64,
    pub owner: i32,
    pub public: bool,
    pub directory: Option<i32>,
}

#[post("/upload", data = "<data>")]
pub fn upload(
    app: AppState,
    mut cookies: Cookies,
    content_type: &ContentType,
    data: Data,
) -> AppResult<Json<Option<i32>>> {
    let app = app.write();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    let size = app.instance.read(conn)?.size_limit;
    let new_file = crate::content::file::multi::from_form(content_type.clone(), data, size)?;
    let hash = app.data.create(&new_file.data, conn)?;
    let id = conn.query("INSERT INTO files (filename, hash, owner, public, directory) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
                        &[&new_file.filename, &hash, &user.id, &new_file.public, &new_file.directory])?
        .first().and_then(|row| row.get(0));
    Ok(Json(id))
}
