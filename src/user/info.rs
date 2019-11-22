use crate::prelude::*;

//
#[get("/user/<id>")]
pub fn get_user(app: AppState, mut cookies: Cookies, id: i32) -> AppResult<Json<Option<UserInfo>>> {
    let inst = app.read();
    Ok(Json(inst.user_repo.read_by_id(id)?.map(|u| u.to_info())))
}

#[get("/me")]
pub fn get_me(app: AppState, mut cookies: Cookies) -> AppResult<Json<Option<UserInfo>>> {
    let inst = app.read();
    match inst.user_from_cookies(&mut cookies) {
        Some(u) => Ok(Json(inst.user_repo.read_by_id(u.id)?.map(|u| u.to_info()))),
        None => Ok(Json(None)),
    }
}
