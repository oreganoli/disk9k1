use rocket::{Request, Response};

pub use user::*;

use crate::prelude::*;

pub mod user;

#[derive(Debug)]
pub enum Error {
    /// The database layer could not be accessed.
    Db,
    /// An unspecified error.
    Other,
    /// Errors related to users.
    User(UserError),
}

impl Error {
    pub fn user<T>(ue: UserError) -> Result<T, Self> {
        Err(Self::User(ue))
    }
    pub fn user_auth<T>(ae: AuthError) -> Result<T, Self> {
        Err(Self::User(UserError::Auth(ae)))
    }
    pub fn reason(&self) -> &str {
        match &self {
            Self::Db => "The database layer could not be accessed or was accessed improperly.",
            Self::User(a) => match a {
                UserError::Auth(auth) => match auth {
                    AuthError::Unauthenticated(_) => "You are not logged in.",
                    AuthError::Unauthorized(_) => "You have insufficient privileges to do this.",
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
            Self::Other => "An unspecified error occurred.",
        }
    }
}

impl rocket::response::Responder<'_> for Error {
    fn respond_to<'r>(self, request: &Request<'r>) -> Result<Response<'static>, Status> {
        let reason = self.reason().to_owned();
        let tera = Tera::new("templates/**/*").unwrap();
        let mut ctx = Context::new();
        ctx.insert("reason", &reason);
        match self {
            Self::User(UserError::Auth(a)) => match a {
                AuthError::Unauthorized(redir) | AuthError::Unauthenticated(redir) => {
                    ctx.insert("login_redirect", &redir);
                    Html(tera.render("PAGE_login.html", &ctx).unwrap()).respond_to(request)
                }
                _ => reason.respond_to(request),
            },
            Self::User(UserError::Registration(_)) => {
                Html(tera.render("PAGE_registration_error.html", &ctx).unwrap()).respond_to(request)
            }
            _ => reason.respond_to(request),
        }
    }
}
