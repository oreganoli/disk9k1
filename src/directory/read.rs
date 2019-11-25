use crate::prelude::*;

use super::Directory;

#[derive(Serialize)]
pub struct DirectoryView {
    directories: Vec<Directory>,
}

impl DirectoryView {
    fn from_vec(vec: Vec<Directory>) -> Self {
        Self { directories: vec }
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
