use crate::prelude::*;

#[derive(Debug, Serialize)]
pub enum UserError {
    Auth(AuthError),
    Deletion(DeletionError),
    Registration(RegistrationError),
    PasswordChange(PasswordChangeError),
    EmailChange(EmailChangeError),
    UsernameChange(UsernameChangeError),
}

#[derive(Debug, Serialize)]
pub enum AuthError {
    BadCredentials,
    Unauthenticated(String),
    Unauthorized(String),
}

#[derive(Debug, Serialize)]
pub enum DeletionError {
    DoesNotExist,
    IsAdmin,
}

#[derive(Debug, Serialize)]
pub enum RegistrationError {
    UsernameTaken,
    UsernameNotGiven,
    EmailNotGiven,
    InvalidEmail,
    PasswordNotConfirmed,
    PasswordNotGiven,
}

#[derive(Debug, Serialize)]
pub enum PasswordChangeError {
    FormIncomplete,
    NotMatching,
    UserNonexistent,
}

#[derive(Debug, Serialize)]
pub enum EmailChangeError {
    Empty,
    UserNonexistent,
}

#[derive(Debug, Serialize)]
pub enum UsernameChangeError {
    Empty,
    Taken,
    UserNonexistent,
}
