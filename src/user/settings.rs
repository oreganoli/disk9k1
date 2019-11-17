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
            self.user_repo.update_password(req.id, req.con).unwrap();
            Ok(())
        }
    }
    pub fn user_change_email(&mut self, id: i32, new_mail: String) -> Result<(), Error> {
        let user = match self.user_repo.read_by_id(id).unwrap() {
            Some(u) => u,
            None => return Error::user_change_email(EmailChangeError::UserNonexistent),
        };
        if new_mail.is_empty() {
            Error::user_change_email(EmailChangeError::Empty)
        } else {
            self.user_repo.update_email(id, new_mail).unwrap();
            Ok(())
        }
    }
}

#[get("/settings")]
pub fn settings(mut cookies: Cookies) -> Result<Page, Error> {
    let inst = instance_read();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Error::user_auth(AuthError::Unauthenticated("settings".to_owned())),
    };
    let mut ctx = Context::new();
    ctx.insert("user", &user.to_info());
    Ok(render("PAGE_settings.html", &ctx))
}

#[post("/change_password", data = "<cp_req>")]
pub fn change_password(
    mut cookies: Cookies,
    cp_req: Form<PasswordChangeRequest>,
) -> Result<Redirect, Error> {
    let mut inst = instance_write();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Error::user_auth(AuthError::Unauthenticated("settings".to_owned())),
    };
    let req = cp_req.into_inner();
    inst.user_change_password(req, &user)
        .map(|_| Redirect::to(uri!(settings)))
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
