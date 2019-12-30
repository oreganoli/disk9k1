use bcrypt::BcryptError;
use rocket::response::Responder;
use rocket::{Request, Response};

use crate::prelude::*;
use crate::user::AuthError;
use crate::user::UserError;

#[derive(Debug, Serialize)]
pub struct ErrorWrapper {
    #[serde(skip)]
    pub status: Status,
    pub name: String,
}

impl<'a> From<r2d2::Error> for ErrorWrapper {
    fn from(e: r2d2::Error) -> Self {
        Self {
            status: Status::InternalServerError,
            name: e.to_string(),
        }
    }
}

impl<'a> From<postgres::Error> for ErrorWrapper {
    fn from(e: postgres::Error) -> Self {
        Self {
            status: Status::InternalServerError,
            name: e.to_string(),
        }
    }
}

impl From<BcryptError> for ErrorWrapper {
    fn from(e: BcryptError) -> Self {
        Self {
            status: Status::InternalServerError,
            name: e.to_string(),
        }
    }
}

impl From<UserError> for ErrorWrapper {
    fn from(e: UserError) -> Self {
        let (status, name) = match e {
            UserError::EmailInvalid => (
                Status::UnprocessableEntity,
                "The email address provided is invalid.",
            ),
            UserError::NameInvalid => (
                Status::UnprocessableEntity,
                "The name provided is invalid.",
            ),
            UserError::PasswordInvalid => (Status::UnprocessableEntity, "Your password must be at least 16 characters long. Length is what matters most - make up a phrase or sentence you will find easy to remember and spell. Remember that you can use spaces and any valid Unicode characters."),
            UserError::PasswordsNotMatching => (Status::UnprocessableEntity, "The passwords provided do not match."),
            UserError::EmailTaken => (Status::Conflict, "This email address is already in use."),
            UserError::NameTaken => (Status::Conflict, "This username is already in use."),
        };
        Self {
            status,
            name: name.to_owned(),
        }
    }
}

impl From<AuthError> for ErrorWrapper {
    fn from(e: AuthError) -> Self {
        match e {
            AuthError::InvalidCredentials => Self {
                status: Status::Unauthorized,
                name: "Invalid password and/or username.".to_owned(),
            },
            AuthError::NotAllowed => Self {
                status: Status::Forbidden,
                name: "You have insufficient privileges to do this or are trying to access a private file you do not own.".to_owned(),
            }
        }
    }
}

impl<'r> Responder<'r> for ErrorWrapper {
    fn respond_to(self, request: &Request<'_>) -> rocket::response::Result<'r> {
        use rocket::response::status::Custom;
        Custom(self.status, Json(self)).respond_to(request)
    }
}
