use crate::error::AuthError::Unauthorized;
use crate::prelude::*;

impl Instance {
    pub fn delete_user(&mut self, id: i32, requester: &User) -> Result<(), Error> {
        let user = self.user_repo.read_by_id(id).unwrap();
        let admin = self.user_repo.read_by_id(1).unwrap().unwrap();
        match user {
            None => Error::user_del(DeletionError::DoesNotExist),
            Some(u) => {
                if u.id == requester.id || admin.id == requester.id {
                    if u.id == admin.id {
                        Error::user_del(DeletionError::IsAdmin)
                    } else {
                        self.user_repo.delete(u.id).unwrap();
                        Ok(())
                    }
                } else {
                    Error::user_auth(Unauthorized(format!("user/{}", u.id)))
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteAccountRequest {
    id: i32,
}

#[post("/delete_account", data = "<da_req>")]
pub fn delete_account(
    app: AppState,
    mut cookies: Cookies,
    da_req: Json<DeleteAccountRequest>,
) -> Result<(), Error> {
    let mut inst = app.write();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Error::user_auth(AuthError::Unauthenticated("me".to_owned())),
    };
    inst.delete_user(da_req.id, &user)
}
