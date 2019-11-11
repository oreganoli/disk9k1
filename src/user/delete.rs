use crate::prelude::*;

pub enum UserDeletionError {
    DoesNotExist,
    IsAdmin,
    NotAllowed,
}

impl Instance {
    pub fn delete_user(&mut self, id: i32, requesting_id: i32) -> Result<(), UserDeletionError> {
        let user = self.user_repo.read_by_id(id).unwrap();
        let admin = self.user_repo.read_by_id(1).unwrap().unwrap();
        match user {
            None => Err(UserDeletionError::DoesNotExist),
            Some(u) => {
                if u.id != requesting_id && requesting_id != admin.id {
                    Err(UserDeletionError::NotAllowed)
                } else if u.id == requesting_id && u.id == admin.id {
                    Err(UserDeletionError::IsAdmin)
                } else {
                    self.user_repo.delete(u.id).unwrap();
                    Ok(())
                }
            }
        }
    }
}

#[derive(FromForm)]
pub struct DeleteAccountRequest {
    id: i32,
}

#[post("/delete_account", data = "<da_req>")]
pub fn delete_account(
    instance: LockState,
    mut cookies: Cookies,
    da_req: Form<DeleteAccountRequest>,
) -> Result<Redirect, Redirect> {
    let mut inst = instance.write().unwrap();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Err(Redirect::to(uri!(crate::login_or_register))),
    };
    match inst.delete_user(da_req.id, user.id) {
        Err(_) => Err(Redirect::to(uri!(crate::login_or_register))),
        Ok(_) => {
            if user.is_admin {
                Ok(Redirect::to(uri!(crate::instance::users::users)))
            } else {
                Ok(Redirect::to(uri!(super::auth::logout)))
            }
        }
    }
}
