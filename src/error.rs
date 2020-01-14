use bcrypt::BcryptError;
use rocket::response::Responder;
use rocket::Request;

use crate::content::data::DataError;
use crate::content::dirs::DirError;
use crate::content::file::FileError;
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

impl From<DirError> for ErrorWrapper {
    fn from(e: DirError) -> Self {
        match e {
            DirError::NameInvalid => Self {
                status: Status::UnprocessableEntity,
                name: "A directory's name may start with a single period and at least one other character and may not contain newlines, tabs, double quotes, or any of the following: '< > ? * | \\ / :'.".to_owned(),
            },
            DirError::NamingConflict => Self {
                status: Status::Conflict,
                name: "A directory cannot have the same name as its siblings in the tree."
                    .to_owned(),
            },
            DirError::Nonexistent => Self {
                status: Status::NotFound,
                name: "The directory in question does not exist.".to_owned(),
            },
            DirError::CyclicParenthood => Self {
                status: Status::UnprocessableEntity,
                name: "A directory cannot be its own parent.".to_owned(),
            },
            DirError::NonexistentParent => Self {
                status: Status::UnprocessableEntity,
                name: "A directory must be either at the top of the filesystem or within an existent directory."
                    .to_owned(),
            }
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
            UserError::AdminDeletion => (Status::Forbidden, "An administrator cannot delete their own account.")
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

impl From<FileError> for ErrorWrapper {
    fn from(e: FileError) -> Self {
        let (status, name) = match e {
            FileError::TooBig => (Status::PayloadTooLarge, "The file is too big."),
            FileError::NamingConflict => (
                Status::Conflict,
                "Files cannot have the same name as others in the same directory.",
            ),
            FileError::NoFileName => (Status::BadRequest, "Files must have a filename."),
            FileError::ImproperForm => (Status::BadRequest, "Your request was malformed."),
        };
        Self {
            status,
            name: name.to_owned(),
        }
    }
}

impl From<DataError> for ErrorWrapper {
    fn from(e: DataError) -> Self {
        match e {
            DataError::TooBig { size, limit } => Self {
                status: Status::PayloadTooLarge,
                name: format!(
                    "The file being uploaded is {}B long, while the maximum size is {}B.",
                    size, limit
                ),
            },
        }
    }
}

impl<'r> Responder<'r> for ErrorWrapper {
    fn respond_to(self, request: &Request<'_>) -> rocket::response::Result<'r> {
        use rocket::response::status::Custom;
        Custom(self.status, Json(self)).respond_to(request)
    }
}
