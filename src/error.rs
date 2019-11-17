use rocket::{Request, Response};

pub use user::*;

use crate::prelude::Status;

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
}

impl rocket::response::Responder<'_> for Error {
    fn respond_to<'r>(self, request: &Request<'r>) -> Result<Response<'static>, Status> {
        let reason = match self {
            Self::Db => "The database layer could not be accessed or was accessed improperly.",
            Self::User(a) => match a {
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
        };
        reason.respond_to(request)
    }
}
