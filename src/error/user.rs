use super::Error;

#[derive(Debug)]
pub enum UserError {
    Auth(AuthError),
    Registration(RegistrationError),
}

#[derive(Debug)]
pub enum AuthError {
    Unauthenticated(Option<&'static str>),
    Unauthorized(Option<&'static str>),
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
