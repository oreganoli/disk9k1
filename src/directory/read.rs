use crate::directory::repo::DirectoryRepository;
use crate::prelude::*;

use super::Directory;

#[derive(Serialize, Clone)]
pub struct DirectoryView {
    info: Option<Directory>,
    directories: Vec<Directory>,
}

impl DirectoryView {
    fn from_id(
        id: i32,
        repo: &Box<dyn DirectoryRepository + Sync + Send>,
    ) -> AppResult<Option<Self>> {
        let children = repo.read_children(id)?;
        Ok(repo.read(id)?.map(|d| Self {
            info: Some(d),
            directories: children,
        }))
    }
    fn from_vec(vec: Vec<Directory>) -> Self {
        Self {
            info: None,
            directories: vec,
        }
    }
}

#[get("/drive")]
pub fn top_for_user(app: AppState, mut cookies: Cookies) -> AppResult<Json<DirectoryView>> {
    let inst = app.read();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return AppError::user_auth(AuthError::Unauthenticated),
    };
    let dir_view = DirectoryView::from_vec(inst.dir_repo.read_top_for_user(&user)?);
    Ok(Json(dir_view))
}

#[get("/drive/<id>")]
pub fn dir_at_id(
    app: AppState,
    mut cookies: Cookies,
    id: i32,
) -> AppResult<Option<Json<DirectoryView>>> {
    let inst = app.read();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return AppError::user_auth(AuthError::Unauthenticated),
    };
    let dir = match DirectoryView::from_id(id, &inst.dir_repo)? {
        Some(dir) => dir,
        None => return Ok(None),
    };
    let info = match &dir.info {
        Some(i) => i.clone(),
        None => return Ok(None),
    };
    if user.id() != info.owner {
        AppError::user_auth(AuthError::Unauthorized)
    } else {
        Ok(Some(Json(dir)))
    }
}
