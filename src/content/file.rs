use std::io::Cursor;
use std::str::FromStr;

use rocket::http::ContentType;
use rocket::response::Redirect;
use rocket::response::{Content, Stream};
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

fn read_file(id: i32, conn: &mut Conn) -> AppResult<Option<File>> {
    Ok(conn
        .query(
            "SELECT id, filename, hash, owner, public, directory FROM files WHERE id = $1",
            &[&id],
        )?
        .first()
        .map(|row| File {
            id: row.get(0),
            filename: row.get(1),
            hash: row.get(2),
            owner: row.get(3),
            public: row.get(4),
            directory: row.get(5),
        }))
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
    let name_valid = conn
        .query(
            include_str!("sql/file/naming_conflicts.sql"),
            &[&new_file.filename, &user.id, &new_file.directory],
        )?
        .first()
        .map_or(false, |row| row.get(0));

    if !name_valid {
        return Err(FileError::NamingConflict.into());
    }
    let hash = app.data.create(&new_file.data, conn)?;
    let id = conn.query("INSERT INTO files (filename, hash, owner, public, directory) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
                        &[&new_file.filename, &hash, &user.id, &new_file.public, &new_file.directory])?
        .first().and_then(|row| row.get(0));
    Ok(Json(id))
}

type FileGet = Option<Content<Stream<Cursor<Vec<u8>>>>>;

fn get_content(hash: i64, conn: &mut Conn) -> AppResult<Option<(Vec<u8>, String)>> {
    Ok(conn
        .query("SELECT data, mimetype FROM data WHERE hash = $1;", &[&hash])?
        .first()
        .map(|row| (row.get(0), row.get(1))))
}

#[get("/file/<id>/<_name>")]
pub fn get_named(
    app: AppState,
    mut cookies: Cookies,
    id: i32,
    _name: String,
) -> AppResult<FileGet> {
    let app = app.read();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn).ok();
    let file = match read_file(id, conn)? {
        Some(f) => f,
        None => return Ok(None),
    };

    match user {
        None => {
            if !file.public {
                return Err(AuthError::NotAllowed.into());
            }
        }
        Some(u) => {
            if !file.public && u.id != file.owner {
                return Err(AuthError::NotAllowed.into());
            }
        }
    }
    let content = get_content(file.hash, conn)?;
    Ok(content.map(|(data, mime)| {
        Content(
            ContentType::from_str(&mime).unwrap_or(ContentType::Binary),
            Stream::from(Cursor::new(data)),
        )
    }))
}

#[get("/file/<id>")]
pub fn get(app: AppState, id: i32) -> AppResult<Option<Redirect>> {
    let app = app.read();
    let conn = &mut app.pool.get()?;
    Ok(read_file(id, conn)?
        .map(|file| Redirect::to(format!("/file/{}/{}", file.id, file.filename))))
}

#[delete("/file/<id>")]
pub fn del(app: AppState, mut cookies: Cookies, id: i32) -> AppResult<()> {
    let app = app.write();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    let file = match read_file(id, conn)? {
        None => return Ok(()),
        Some(f) => f,
    };
    if user.id != file.owner {
        Err(AuthError::NotAllowed.into())
    } else {
        conn.execute("DELETE FROM files WHERE id = $1", &[&id])?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct FileRename {
    name: String,
}

#[put("/rename_file/<id>", data = "<ren>")]
pub fn rename_file(
    app: AppState,
    mut cookies: Cookies,
    id: i32,
    ren: Json<FileRename>,
) -> AppResult<()> {
    let app = app.write();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    let owner_same: bool = conn
        .query_one(
            "SELECT (SELECT owner FROM files where id = $1) = $2;",
            &[&id, &user.id],
        )?
        .get(0);
    if !owner_same {
        return Err(AuthError::NotAllowed.into());
    }
    let parent_dir: Option<i32> = conn
        .query_one("SELECT directory FROM files WHERE id = $1", &[&id])?
        .get(0);
    let name_valid: bool = conn
        .query_one(
            include_str!("sql/file/naming_conflicts.sql"),
            &[&ren.name, &user.id, &parent_dir],
        )?
        .get(0);
    if name_valid {
        conn.execute(
            "UPDATE files SET filename = $2 WHERE id = $1",
            &[&id, &ren.name],
        )?;
        Ok(())
    } else {
        Err(FileError::NamingConflict.into())
    }
}
