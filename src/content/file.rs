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

#[post("/upload", data = "<data>")]
pub fn upload(
    app: AppState,
    mut cookies: Cookies,
    content_type: &ContentType,
    data: Data,
) -> AppResult<()> {
    let app = app.write();
    let conn = &mut app.pool.get()?;
    let size = app.instance.read(conn)?.size_limit;
    let file = crate::content::file::multi::from_form(content_type.clone(), data, size)?;
    Ok(())
}
