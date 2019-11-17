use super::Error;

#[derive(Debug)]
pub enum UserError {
    Registration(RegistrationError),
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
