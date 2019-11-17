use crate::error::UserError::Registration;
use crate::prelude::*;

#[derive(FromForm)]
pub struct RegistrationRequest {
    username: String,
    email: String,
    password: String,
    password_con: String,
}

/// Generates a quick-access token for a user name.
/// DO NOT USE THIS IN PRODUCTION!
#[cfg(debug_assertions)] // will break if you try anyway
pub fn generate_token(name: &str) -> String {
    format!("{}-PLACEHOLDER", name)
}

fn is_valid_email(email: &str) -> bool {
    true //TODO: implement actual regex-based checking
}

impl Instance {
    pub fn register_user(&mut self, req: RegistrationRequest) -> Result<String, Error> {
        if self
            .user_repo
            .read_by_name(req.username.clone())
            .unwrap()
            .is_some()
        {
            Error::user(Registration(RegistrationError::UsernameTaken))
        } else if req.username.is_empty() {
            Error::user(Registration(RegistrationError::UsernameNotGiven))
        } else if req.email.is_empty() {
            Error::user(Registration(RegistrationError::EmailNotGiven))
        } else if !is_valid_email(&req.email) {
            Error::user(Registration(RegistrationError::InvalidEmail))
        } else if req.password != req.password_con {
            Error::user(Registration(RegistrationError::PasswordNotConfirmed))
        } else if req.password.is_empty() {
            Error::user(Registration(RegistrationError::PasswordNotGiven))
        } else {
            let token = generate_token(&req.username);
            self.user_repo
                .create(NewUser::new(
                    req.username,
                    req.email,
                    req.password,
                    token.clone(),
                    false,
                ))
                .unwrap();
            Ok(token)
        }
    }
}

#[post("/register", data = "<reg_req>")]
pub fn register(reg_req: Form<RegistrationRequest>) -> Result<Page, Error> {
    let mut inst = instance_write();
    let mut ctx = Context::new();
    inst.register_user(reg_req.into_inner()).map(|token| {
        ctx.insert("token", &token);
        render("PAGE_successful_registration.html", &ctx)
    })
}
