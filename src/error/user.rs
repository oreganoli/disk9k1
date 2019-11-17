use super::Error;

#[derive(Debug)]
pub enum UserError {
    Auth(AuthError),
    Registration(RegistrationError),
}

#[derive(Debug)]
pub enum AuthError {
    Unauthenticated(&'static str),
    Unauthorized(&'static str),
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
