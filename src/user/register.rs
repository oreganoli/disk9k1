use crate::prelude::*;

#[derive(FromForm)]
pub struct RegistrationRequest {
    username: String,
    email: String,
    password: String,
    password_con: String,
}

pub enum RegistrationError {
    UsernameTaken,
    UsernameNotGiven,
    PasswordNotConfirmed,
    PasswordNotGiven,
}

/// Generates a quick-access token for a user name.
/// DO NOT USE THIS IN PRODUCTION!
#[cfg(debug_assertions)] // will break if you try anyway
pub fn generate_token(name: &str) -> String {
    format!("{}-PLACEHOLDER", name)
}

impl Instance {
    pub fn register_user(&mut self, req: RegistrationRequest) -> Result<String, RegistrationError> {
        if self
            .user_repo
            .read_by_name(req.username.clone())
            .unwrap()
            .is_some()
        {
            Err(RegistrationError::UsernameTaken)
        } else if req.username.is_empty() {
            Err(RegistrationError::UsernameNotGiven)
        } else if req.password != req.password_con {
            Err(RegistrationError::PasswordNotConfirmed)
        } else if req.password.is_empty() {
            Err(RegistrationError::PasswordNotGiven)
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
pub fn register(
    instance: LockState,
    tera: TeraState,
    reg_req: Form<RegistrationRequest>,
) -> Result<Page, Page> {
    let mut inst = instance.write().unwrap();
    let mut ctx = Context::new();
    ctx.insert("instance", &inst.ins_repo.get().unwrap());
    match inst.register_user(reg_req.into_inner()) {
        Err(RegistrationError::UsernameTaken) => {
            ctx.insert("reason", "The username is already in use.");
            Err(tera.html("PAGE_registration_error.html", &ctx))
        }
        Err(RegistrationError::UsernameNotGiven) => {
            ctx.insert("reason", "No username was given.");
            Err(tera.html("PAGE_registration_error.html", &ctx))
        }
        Err(RegistrationError::PasswordNotGiven) => {
            ctx.insert("reason", "No password was given.");
            Err(tera.html("PAGE_registration_error.html", &ctx))
        }
        Err(RegistrationError::PasswordNotConfirmed) => {
            ctx.insert("reason", "The passwords provided do not match.");
            Err(tera.html("PAGE_registration_error.html", &ctx))
        }
        Ok(token) => {
            ctx.insert("token", &token);
            Ok(tera.html("PAGE_successful_registration.html", &ctx))
        }
    }
}
