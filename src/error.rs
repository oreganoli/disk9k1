use rocket::response::ResponseBuilder;
use rocket::{Request, Response};

use crate::prelude::Status;

#[derive(Debug)]
pub enum Error {
    /// The database layer could not be accessed.
    Db,
    /// An unspecified error.
    Other,
}

impl rocket::response::Responder<'_> for Error {
    fn respond_to<'r>(self, request: &Request<'r>) -> Result<Response<'static>, Status> {
        let reason = match self {
            Self::Db => "The database layer could not be accessed or was accessed improperly.",
            Self::Other => "An unspecified error occurred.",
        };
        reason.respond_to(request)
    }
}
