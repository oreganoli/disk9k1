use crate::prelude::*;

#[derive(FromForm)]
pub struct PasswordChangeRequest {
    pub id: i32,
    pub new: String,
    pub con: String,
}

impl Instance {
    pub fn user_change_password(
        &mut self,
        req: PasswordChangeRequest,
        requester: &User,
    ) -> Result<(), Error> {
        if requester.id != req.id {
            return Error::user_auth(AuthError::Unauthorized("settings".to_owned()));
        }
        if req.con != req.new {
            Error::user_change_pass(PasswordChangeError::NotMatching)
        } else if req.new.is_empty() || req.con.is_empty() {
            Error::user_change_pass(PasswordChangeError::FormIncomplete)
        } else {
            self.user_repo.update_password(req.id, req.con)
        }
    }
    pub fn user_change_email(&mut self, id: i32, new_mail: String) -> Result<(), Error> {
        if self.user_repo.read_by_id(id)?.is_none() {
            return Error::user_change_email(EmailChangeError::UserNonexistent);
        }
        if new_mail.is_empty() {
            Error::user_change_email(EmailChangeError::Empty)
        } else {
            self.user_repo.update_email(id, new_mail)
        }
    }
    pub fn user_change_username(
        &mut self,
        id: i32,
        req: UsernameChangeRequest,
    ) -> Result<(), Error> {
        if req.name.is_empty() {
            return Error::user_change_name(UsernameChangeError::Empty);
        }
        if self.user_repo.read_by_id(id)?.is_none() {
            return Error::user_change_name(UsernameChangeError::UserNonexistent);
        }
        if self.user_repo.read_by_name(req.name.clone())?.is_some() {
            return Error::user_change_name(UsernameChangeError::Taken);
        }
        self.user_repo.update_name(id, req.name)
    }
}

#[post("/change_password", data = "<cp_req>")]
pub fn change_password(
    mut cookies: Cookies,
    cp_req: Form<PasswordChangeRequest>,
) -> Result<Redirect, Error> {
    unimplemented!()
}

#[derive(FromForm)]
pub struct EmailChangeRequest {
    pub email: String,
}

#[post("/change_email", data = "<ec_req>")]
pub fn change_email(
    mut cookies: Cookies,
    ec_req: Form<EmailChangeRequest>,
) -> Result<Redirect, Error> {
    let mut inst = instance_write();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Error::user_auth(AuthError::Unauthenticated("settings".to_owned())),
    };
    inst.user_change_email(user.id(), ec_req.email.clone())
        .map(|_| Redirect::to(uri!(super::info::get_me)))
}

#[derive(FromForm)]
pub struct UsernameChangeRequest {
    pub name: String,
}

#[post("/change_username", data = "<uc_req>")]
pub fn change_username(
    mut cookies: Cookies,
    uc_req: Form<UsernameChangeRequest>,
) -> Result<Redirect, Error> {
    unimplemented!()
}
