use crate::prelude::*;

#[derive(FromForm)]
pub struct PasswordChangeRequest {
    pub id: i32,
    pub new: String,
    pub con: String,
}

pub enum PasswordChangeError {
    FormIncomplete,
    NotAllowed,
    NotMatching,
    UserNonexistent,
}

pub enum EmailChangeError {
    Empty,
    UserNonexistent,
}

impl Instance {
    pub fn user_change_password(
        &mut self,
        req: PasswordChangeRequest,
        requester: &User,
    ) -> Result<(), PasswordChangeError> {
        if requester.id != req.id {
            return Err(PasswordChangeError::NotAllowed);
        }
        if req.con != req.new {
            Err(PasswordChangeError::NotMatching)
        } else if req.new.is_empty() || req.con.is_empty() {
            Err(PasswordChangeError::FormIncomplete)
        } else {
            self.user_repo.update_password(req.id, req.con).unwrap();
            Ok(())
        }
    }
    //    pub fn user_change_email(&mut self, id: i32, new_mail: String) -> Result<(), EmailChangeError> {
    //        let user = match self.user_repo.read_by_id(id).unwrap() {
    //            Some(u) => u,
    //            None => return Err(EmailChangeError::UserNonexistent)
    //        };
    //        if new_mail.is_empty() {
    //            Err(EmailChangeError::Empty)
    //        } else {
    //            self.user_repo.upda
    //        }
    //    }
}

#[get("/settings")]
pub fn settings(
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
    Ok(tera.html("PAGE_settings.html", &ctx))
}

#[post("/change_password", data = "<cp_req>")]
pub fn change_password(
    instance: LockState,
    mut cookies: Cookies,
    cp_req: Form<PasswordChangeRequest>,
) -> Result<Redirect, Redirect> {
    let mut inst = instance.write().unwrap();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Err(Redirect::to(uri!(super::auth::login))),
    };
    let req = cp_req.into_inner();
    let passwd = req.new.clone();
    match inst.user_change_password(req, &user) {
        Ok(_) => {
            cookies.add_private(Cookie::new("password", passwd));
            Ok(Redirect::to(uri!(settings)))
        }
        Err(PasswordChangeError::UserNonexistent) => Err(Redirect::to(uri!(super::auth::login))),
        Err(PasswordChangeError::NotAllowed) => Err(Redirect::to(uri!(super::auth::login))),
        _ => Err(Redirect::to(uri!(settings))),
    }
}

//#[derive(FromForm)]
//pub struct EmailChangeRequest {
//    pub email: String,
//}
//
//#[post("/change_email", data = "<ec_req>")]
//pub fn change_email(
//    instance: LockState,
//    mut cookies: Cookies,
//    ec_req: Form<EmailChangeRequest>,
//) -> Result<Redirect, Redirect> {
//    let inst = instance.write().unwrap();
//}
