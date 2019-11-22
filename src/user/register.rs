use crate::error::UserError::Registration;
use crate::prelude::*;

#[derive(Deserialize)]
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
    pub fn register_user(&mut self, req: RegistrationRequest) -> AppResult<()> {
        if self
            .user_repo
            .read_by_name(req.username.clone())
            .unwrap()
            .is_some()
        {
            AppError::user(Registration(RegistrationError::UsernameTaken))
        } else if req.username.is_empty() {
            AppError::user(Registration(RegistrationError::UsernameNotGiven))
        } else if req.email.is_empty() {
            AppError::user(Registration(RegistrationError::EmailNotGiven))
        } else if !is_valid_email(&req.email) {
            AppError::user(Registration(RegistrationError::InvalidEmail))
        } else if req.password != req.password_con {
            AppError::user(Registration(RegistrationError::PasswordNotConfirmed))
        } else if req.password.is_empty() {
            AppError::user(Registration(RegistrationError::PasswordNotGiven))
        } else {
            let token = generate_token(&req.username);
            self.user_repo
                .create(NewUser::new(req.username, req.email, req.password, false))
                .unwrap();
            Ok(())
        }
    }
}

#[post("/register", data = "<reg_req>")]
pub fn register(app: AppState, reg_req: Json<RegistrationRequest>) -> AppResult<()> {
    let mut inst = app.write();
    inst.register_user(reg_req.into_inner())
}
