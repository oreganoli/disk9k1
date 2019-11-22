use std::borrow::BorrowMut;

use rocket::{Request, Response};
use serde::export::Formatter;

pub use error::*;
pub use instance::*;
pub use user::*;

use crate::prelude::*;

pub mod error;
pub mod instance;
pub mod user;

#[derive(Debug)]
pub enum AppError {
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

impl AppError {
    pub fn dir<T>(de: DirectoryError) -> Result<T, Self> {
        Err(Self::Dir(de))
    }
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
    pub fn instance<T>(ie: InstanceError) -> Result<T, Self> {
        Err(Self::Instance(ie))
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Db => "A database error occurred.".to_owned(),
                Self::Dir(d) => format!("{}", d),
                Self::Instance(i) => format!("{}", i),
                Self::User(u) => format!("{}", u),
                Self::Other => "Another error occurred.".to_owned(),
            }
        )
    }
}

impl rocket::response::Responder<'_> for AppError {
    fn respond_to<'r>(self, request: &Request<'r>) -> Result<Response<'static>, Status> {
        status::BadRequest(Some(Json(format!("{}", self)))).respond_to(request)
    }
}
