#[derive(Debug)]
pub enum UserError {
    Auth(AuthError),
    Deletion(DeletionError),
    Registration(RegistrationError),
}

#[derive(Debug)]
pub enum AuthError {
    BadCredentials,
    Unauthenticated(String),
    Unauthorized(String),
}

#[derive(Debug)]
pub enum DeletionError {
    DoesNotExist,
    IsAdmin,
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
