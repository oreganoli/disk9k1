use core::fmt::Error;

use serde::export::Formatter;

use crate::prelude::*;

#[derive(Debug)]
pub enum UserError {
    Auth(AuthError),
    Deletion(DeletionError),
    Registration(RegistrationError),
    PasswordChange(PasswordChangeError),
    EmailChange(EmailChangeError),
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Auth(a) => format!("{}", a),
                Self::Deletion(a) => format!("{}", a),
                Self::Registration(a) => format!("{}", a),
                Self::PasswordChange(a) => format!("{}", a),
                Self::EmailChange(a) => format!("{}", a),
            }
        )
    }
}

#[derive(Debug)]
pub enum AuthError {
    BadCredentials,
    Unauthenticated,
    Unauthorized,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Self::BadCredentials => "Invalid username and/or password.",
                Self::Unauthenticated => "You are not logged in.",
                Self::Unauthorized => "You are not allowed to do this.",
            }
        )
    }
}

#[derive(Debug)]
pub enum DeletionError {
    DoesNotExist,
    IsAdmin,
}

impl Display for DeletionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Self::DoesNotExist => "No such user exists.",
                Self::IsAdmin => "An admin account cannot be deleted.",
            }
        )
    }
}

#[derive(Debug)]
pub enum RegistrationError {
    UsernameTaken,
    UsernameNotGiven,
    EmailNotGiven,
    InvalidEmail,
    PasswordNotConfirmed,
    PasswordNotGiven,
}

impl Display for RegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Self::UsernameTaken => "This username is already taken.",
                Self::UsernameNotGiven => "No username was provided.",
                Self::EmailNotGiven => "No email address was provided.",
                Self::InvalidEmail => "The email address given is invalid.",
                Self::PasswordNotConfirmed => "The password was not confirmed.",
                Self::PasswordNotGiven => "No password was provided.",
            }
        )
    }
}

#[derive(Debug)]
pub enum PasswordChangeError {
    FormIncomplete,
    NotMatching,
}

impl Display for PasswordChangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Self::FormIncomplete => "One must provide both a new password and a confirmation.",
                Self::NotMatching => "The passwords given do not match.",
            }
        )
    }
}

#[derive(Debug)]
pub enum EmailChangeError {
    Empty,
    InvalidEmail,
}

impl Display for EmailChangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => "An email address must be provided.",
                Self::InvalidEmail => "The email address given is invalid.",
            }
        )
    }
}
