#[derive(Debug)]
pub enum UserError {
    Auth(AuthError),
    Deletion(DeletionError),
    Registration(RegistrationError),
    PasswordChange(PasswordChangeError),
    EmailChange(EmailChangeError),
    UsernameChange(UsernameChangeError),
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

#[derive(Debug)]
pub enum PasswordChangeError {
    FormIncomplete,
    NotMatching,
    UserNonexistent,
}

#[derive(Debug)]
pub enum EmailChangeError {
    Empty,
    UserNonexistent,
}

#[derive(Debug)]
pub enum UsernameChangeError {
    Empty,
    Taken,
    UserNonexistent,
}
