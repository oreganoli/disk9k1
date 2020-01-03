use crate::prelude::*;

use super::ContentRepo;

#[derive(Serialize)]
pub struct DirView {
    id: i32,
    name: String,
    owner: i32,
    parent: Option<i32>,
    children: Vec<Directory>, // TODO add child files
}

#[derive(Serialize)]
pub struct Directory {
    id: i32,
    name: String,
    owner: i32,
    parent: Option<i32>,
}

impl Directory {
    pub fn into_view(self, repo: &ContentRepo, conn: &mut Conn) -> AppResult<DirView> {
        let children = repo.read_children(self.id, conn)?;
        Ok(DirView {
            id: self.id,
            name: self.name,
            owner: self.owner,
            parent: self.parent,
            children,
        })
    }
}
#[derive(Deserialize)]
pub struct NewDir {
    name: String,
    owner: i32,
    parent: Option<i32>,
}

pub(crate) enum DirError {
    NameInvalid,
    NamingConflict,
    CyclicParenthood,
    Nonexistent,
    NonexistentParent,
}

impl ContentRepo {
    pub fn create(&self, new: NewDir, conn: &mut Conn) -> AppResult<()> {
        if !self.folder_regex.is_match(&new.name) {
            return Err(DirError::NameInvalid.into());
        }
        // check for naming conflicts
        let naming_ok = conn
            .query(
                include_str!("sql/dirs/naming_conflicts.sql"),
                &[&new.name, &new.owner, &new.parent],
            )?
            .first()
            .map_or(false, |row| row.get(0));
        let parent_exists = match new.parent {
            None => true,
            Some(parent) => conn
                .query(include_str!("sql/dirs/exists_parent.sql"), &[&parent])?
                .first()
                .map_or(false, |row| row.get(0)),
        };
        //check for ownership conflict
        let ownership_ok = match new.parent {
            None => true,
            _ => conn
                .query(
                    include_str!("sql/dirs/owner_same.sql"),
                    &[&new.owner, &new.parent],
                )?
                .first()
                .map_or(false, |row| row.get(0)),
        };
        if !(parent_exists) {
            return Err(DirError::NonexistentParent.into());
        }
        if !(ownership_ok) {
            return Err(AuthError::NotAllowed.into());
        }
        if !(naming_ok) {
            return Err(DirError::NamingConflict.into());
        }
        conn.execute(
            include_str!("sql/dirs/create.sql"),
            &[&new.name, &new.owner, &new.parent],
        )?;
        Ok(())
    }
    pub fn read(&self, id: i32, conn: &mut Conn) -> AppResult<Option<Directory>> {
        let dir = conn
            .query(include_str!("sql/dirs/read.sql"), &[&id])?
            .first()
            .map(|row| Directory {
                id: row.get(0),
                name: row.get(1),
                owner: row.get(2),
                parent: row.get(3),
            });
        Ok(dir)
    }
    pub fn read_children(&self, id: i32, conn: &mut Conn) -> AppResult<Vec<Directory>> {
        let dirs = conn
            .query(include_str!("sql/dirs/read_children.sql"), &[&id])?
            .iter()
            .map(|row| Directory {
                id: row.get(0),
                name: row.get(1),
                owner: row.get(2),
                parent: row.get(3),
            })
            .collect::<Vec<_>>();
        Ok(dirs)
    }
    pub fn read_top(&self, user_id: i32, conn: &mut Conn) -> AppResult<Vec<Directory>> {
        let dirs = conn
            .query(include_str!("sql/dirs/read_top_user.sql"), &[&user_id])?
            .iter()
            .map(|row| Directory {
                id: row.get(0),
                name: row.get(1),
                owner: row.get(2),
                parent: row.get(3),
            })
            .collect::<Vec<_>>();
        Ok(dirs)
    }
    pub fn update_name(&self, id: i32, name: &str, conn: &mut Conn) -> AppResult<()> {
        if !self.folder_regex.is_match(name) {
            return Err(DirError::NameInvalid.into());
        }
        let dir = match self.read(id, conn)? {
            None => return Err(DirError::Nonexistent.into()),
            Some(d) => d,
        };
        let validity = conn
            .query(
                include_str!("sql/dirs/naming_conflicts.sql"),
                &[&name, &dir.owner, &dir.parent],
            )?
            .first()
            .map_or(false, |row| row.get(0));
        if !validity {
            Err(DirError::NamingConflict.into())
        } else {
            conn.execute(include_str!("sql/dirs/update_name.sql"), &[&id, &name])?;
            Ok(())
        }
    }
}

#[get("/drive")]
pub fn get_top(app: AppState, mut cookies: Cookies) -> AppResult<Json<DirView>> {
    let app = app.read();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    let children = app.content.read_top(user.id, conn)?;
    let view = DirView {
        id: 0,
        name: "/".to_owned(),
        owner: user.id,
        parent: None,
        children,
    };
    Ok(Json(view))
}

#[get("/drive/<id>")]
pub fn get(app: AppState, mut cookies: Cookies, id: i32) -> AppResult<Option<Json<DirView>>> {
    let app = app.read();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    let dir = match app.content.read(id, conn)? {
        None => return Ok(None),
        Some(d) => d,
    };
    if user.id != dir.owner {
        return Err(AuthError::NotAllowed.into());
    }
    let view = dir.into_view(&app.content, conn)?;
    Ok(Some(Json(view)))
}

#[post("/drive", data = "<new>")]
pub fn post(app: AppState, mut cookies: Cookies, new: Json<NewDir>) -> AppResult<()> {
    let app = app.write();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    if user.id != new.owner {
        return Err(AuthError::NotAllowed.into());
    }
    app.content.create(new.into_inner(), conn)
}
