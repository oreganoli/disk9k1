use crate::error::AuthError::Unauthorized;
use crate::prelude::*;

impl Instance {
    pub fn delete_user(&mut self, id: i32, requester: &User) -> AppResult<()> {
        let user = self.user_repo.read_by_id(id).unwrap();
        let admin = self.user_repo.read_by_id(1).unwrap().unwrap();
        match user {
            None => AppError::user_del(DeletionError::DoesNotExist),
            Some(u) => {
                if u.id == requester.id || admin.id == requester.id {
                    if u.id == admin.id {
                        AppError::user_del(DeletionError::IsAdmin)
                    } else {
                        self.user_repo.delete(u.id).unwrap();
                        Ok(())
                    }
                } else {
                    AppError::user_auth(Unauthorized)
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteAccountRequest {
    id: i32,
}

#[delete("/delete_account", data = "<da_req>")]
pub fn delete_account(
    app: AppState,
    mut cookies: Cookies,
    da_req: Json<DeleteAccountRequest>,
) -> AppResult<()> {
    let mut inst = app.write();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return AppError::user_auth(AuthError::Unauthenticated).into(),
    };
    inst.delete_user(da_req.id, &user)
}
