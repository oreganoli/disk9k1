use crate::prelude::*;

use super::Directory;

#[get("/drive")]
pub fn top_for_user(app: AppState, mut cookies: Cookies) -> AppResult<Json<Vec<Directory>>> {
    let inst = app.read();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return AppError::user_auth(AuthError::Unauthenticated),
    };
    Ok(Json(inst.dir_repo.read_top_for_user(&user)?))
}
