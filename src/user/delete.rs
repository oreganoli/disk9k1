use crate::prelude::*;

pub enum UserDeletionError {
    DoesNotExist,
    IsAdmin,
    NotAllowed,
}

impl Instance {
    pub fn delete_user(&mut self, id: i32, requester: &User) -> Result<(), UserDeletionError> {
        let user = self.user_repo.read_by_id(id).unwrap();
        let admin = self.user_repo.read_by_id(1).unwrap().unwrap();
        match user {
            None => Err(UserDeletionError::DoesNotExist),
            Some(u) => {
                if u.id == requester.id || admin.id == requester.id {
                    if u.id == admin.id {
                        Err(UserDeletionError::IsAdmin)
                    } else {
                        self.user_repo.delete(u.id).unwrap();
                        Ok(())
                    }
                } else {
                    Err(UserDeletionError::NotAllowed)
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
        None => return Err(Redirect::to(uri!(super::auth::login))),
    };
    match inst.delete_user(da_req.id, &user) {
        Err(_) => Err(Redirect::to(uri!(super::auth::login))),
        Ok(_) => {
            if user.is_admin {
                Ok(Redirect::to(uri!(crate::instance::users::users)))
            } else {
                Ok(Redirect::to(uri!(super::auth::logout)))
            }
        }
    }
}

#[get("/delete_account_confirm")]
pub fn del_acc_confirm(
    instance: LockState,
    tera: TeraState,
    mut cookies: Cookies,
) -> Result<Page, Redirect> {
    let inst = instance.read().unwrap();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Err(Redirect::to(uri!(super::auth::login))),
    };
    let mut ctx = Context::new();
    ctx.insert("instance", &inst.ins_repo.get().unwrap());
    ctx.insert("user", &user.to_info());
    Ok(tera.html("PAGE_delete_account_confirm.html", &ctx))
}
