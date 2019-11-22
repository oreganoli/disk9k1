use crate::prelude::*;

#[derive(Deserialize)]
pub struct PasswordChangeRequest {
    pub new: String,
    pub con: String,
}

impl Instance {
    pub fn user_change_password(
        &mut self,
        req: PasswordChangeRequest,
        requester: &User,
    ) -> AppResult<()> {
        if req.con != req.new {
            AppError::user_change_pass(PasswordChangeError::NotMatching)
        } else if req.new.is_empty() || req.con.is_empty() {
            AppError::user_change_pass(PasswordChangeError::FormIncomplete)
        } else {
            self.user_repo.update_password(requester.id(), req.con)
        }
    }
    pub fn user_change_email(
        &mut self,
        req: EmailChangeRequest,
        requester: &User,
    ) -> AppResult<()> {
        if req.email.is_empty() {
            AppError::user_change_email(EmailChangeError::Empty)
        } else {
            self.user_repo.update_email(requester.id, req.email)
        }
    }
}

#[put("/change_password", data = "<cp_req>")]
pub fn change_password(
    app: AppState,
    mut cookies: Cookies,
    cp_req: Json<PasswordChangeRequest>,
) -> AppResult<()> {
    let mut inst = app.write();
    let new = cp_req.new.clone();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return AppError::user_auth(AuthError::Unauthenticated),
    };
    inst.user_change_password(cp_req.into_inner(), &user)
        .map(|_| cookies.add_private(Cookie::new("password", new)))
}

#[derive(Deserialize)]
pub struct EmailChangeRequest {
    pub email: String,
}

#[put("/change_email", data = "<ec_req>")]
pub fn change_email(
    app: AppState,
    mut cookies: Cookies,
    ec_req: Json<EmailChangeRequest>,
) -> AppResult<()> {
    let mut inst = app.write();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return AppError::user_auth(AuthError::Unauthenticated),
    };
    inst.user_change_email(ec_req.into_inner(), &user)
}
