use std::borrow::BorrowMut;

use rocket::{Request, Response};

pub use error::*;
pub use instance::*;
pub use user::*;

use crate::prelude::*;

pub mod error;
pub mod instance;
pub mod user;

#[derive(Debug)]
pub enum Error {
    /// The database layer could not be accessed.
    Db,
    Dir(DirectoryError),
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
    pub fn user_change_pass<T>(pe: PasswordChangeError) -> Result<T, Self> {
        Err(Self::User(UserError::PasswordChange(pe)))
    }
    pub fn user_change_email<T>(mc: EmailChangeError) -> Result<T, Self> {
        Err(Self::User(UserError::EmailChange(mc)))
    }
    pub fn user_change_name<T>(uc: UsernameChangeError) -> Result<T, Self> {
        Err(Self::User(UserError::UsernameChange(uc)))
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
                UserError::PasswordChange(pas) => match pas {
                    PasswordChangeError::FormIncomplete => "No password was provided.",
                    PasswordChangeError::NotMatching => "The passwords provided do not match.",
                    PasswordChangeError::UserNonexistent => "This user does not exist.",
                },
                UserError::EmailChange(emc) => match emc {
                    EmailChangeError::Empty => "No email address was provided.",
                    EmailChangeError::UserNonexistent => "This user does not exist.",
                },
                UserError::UsernameChange(uc) => match uc {
                    UsernameChangeError::Empty => "No new username was provided.",
                    UsernameChangeError::Taken => "That username is already taken.",
                    UsernameChangeError::UserNonexistent => "This user does not exist.",
                },
            },
            Self::Instance(a) => match a {
                InstanceError::NameEmpty => "A Disk9k1 instance must have a name.",
                InstanceError::NegativeSizeLimit => "The size limit on files cannot be negative.",
            },
            Self::Dir(d) => match d {
                DirectoryError::CyclicReference => "There was an attempt to create a directory inside itself.",
                DirectoryError::Nonexistent => "No such directory exists.",
                DirectoryError::NotSameOwner => "There was an attempt to create a directory in a directory not owned by the same user."
            }
            Self::Other => "An unspecified error occurred.",
        }
    }
}

impl rocket::response::Responder<'_> for Error {
    fn respond_to<'r>(self, request: &Request<'r>) -> Result<Response<'static>, Status> {
        unimplemented!()
    }
}
