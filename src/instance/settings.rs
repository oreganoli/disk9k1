use crate::prelude::*;

impl Instance {
    pub fn set_instance_data(&mut self, new_data: InstanceData, requester: &User) -> AppResult<()> {
        if new_data.name.is_empty() {
            return AppError::instance(InstanceError::NameEmpty).into();
        }
        if new_data.size_limit < 0 {
            return AppError::instance(InstanceError::NegativeSizeLimit).into();
        }
        if requester.is_admin() {
            self.ins_repo.set(new_data).unwrap();
            Ok(())
        } else {
            AppError::user_auth(AuthError::Unauthorized).into()
        }
    }
}

#[put("/modify_instance", data = "<ins_req>")]
pub fn modify_instance(
    app: AppState,
    mut cookies: Cookies,
    ins_req: Json<InstanceData>,
) -> AppResult<Json<()>> {
    let mut inst = app.write();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return AppError::user_auth(AuthError::Unauthenticated).into(),
    };
    inst.set_instance_data(ins_req.into_inner(), &user)
        .map(|_| Json(()))
}
