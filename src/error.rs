use std::borrow::BorrowMut;

use rocket::{Request, Response};

pub use instance::*;
pub use user::*;

use crate::prelude::*;

pub mod instance;
pub mod user;

#[derive(Debug)]
pub enum Error {
    /// The database layer could not be accessed.
    Db,
    ///
    Instance(InstanceError),
    /// Errors related to users.
    User(UserError),
    /// An unspecified error.
    Other,
}

impl Error {
    pub fn user<T>(ue: UserError) -> Result<T, Self> {
        Err(Self::User(ue))
    }
    pub fn user_auth<T>(ae: AuthError) -> Result<T, Self> {
        Err(Self::User(UserError::Auth(ae)))
    }
    pub fn user_del<T>(de: DeletionError) -> Result<T, Self> {
        Err(Self::User(UserError::Deletion(de)))
    }
    pub fn instance<T>(ie: InstanceError) -> Result<T, Self> {
        Err(Self::Instance(ie))
    }
    pub fn reason(&self) -> &str {
        match &self {
            Self::Db => "The database layer could not be accessed or was accessed improperly.",
            Self::User(a) => match a {
                UserError::Auth(auth) => match auth {
                    AuthError::BadCredentials => "Invalid username and/or password.",
                    AuthError::Unauthenticated(_) => "You are not logged in.",
                    AuthError::Unauthorized(_) => "You have insufficient privileges to do this.",
                },
                UserError::Deletion(del) => match del {
                    DeletionError::DoesNotExist => "You are trying to delete a nonexistent user.",
                    DeletionError::IsAdmin => "The admin account cannot be removed.",
                },
                UserError::Registration(reg) => match reg {
                    RegistrationError::UsernameTaken => "This username is taken.",
                    RegistrationError::UsernameNotGiven => "No username was provided",
                    RegistrationError::PasswordNotConfirmed => "The passwords do not match.",
                    RegistrationError::PasswordNotGiven => "No password was provided.",
                    RegistrationError::InvalidEmail => "The email address provided is not valid.",
                    RegistrationError::EmailNotGiven => "No email address was provided.",
                },
            },
            Self::Instance(a) => match a {
                InstanceError::NameEmpty => "A Disk9k1 instance must have a name.",
                InstanceError::NegativeSizeLimit => "The size limit on files cannot be negative.",
            },
            Self::Other => "An unspecified error occurred.",
        }
    }
}

impl rocket::response::Responder<'_> for Error {
    fn respond_to<'r>(self, request: &Request<'r>) -> Result<Response<'static>, Status> {
        let reason = self.reason().to_owned();
        let mut ctx = Context::new();
        let instance: RwLockReadGuard<'static, Instance> = instance_read();
        ctx.insert("reason", &reason);
        let user = instance.user_from_cookies(request.cookies().borrow_mut());
        let info = match user.as_ref() {
            Some(u) => Some(u.to_info()),
            None => None,
        };
        ctx.insert("user", &info);
        match self {
            Self::User(UserError::Auth(a)) => match a {
                AuthError::Unauthorized(redir) | AuthError::Unauthenticated(redir) => {
                    ctx.insert("login_redirect", &redir);
                    render("PAGE_login.html", &ctx).respond_to(request)
                }
                AuthError::BadCredentials => render("PAGE_login.html", &ctx).respond_to(request),
            },
            Self::User(UserError::Registration(_)) => {
                render("PAGE_registration_error.html", &ctx).respond_to(request)
            }
            Self::Instance(_) => render("PAGE_panel.html", &ctx).respond_to(request),
            _ => reason.respond_to(request),
        }
    }
}
