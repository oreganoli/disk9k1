use rocket::response::Responder;
use rocket::{Request, Response};

use crate::prelude::*;

#[derive(Debug, Serialize)]
pub struct ErrorWrapper {
    #[serde(skip)]
    pub status: Status,
    pub name: String,
}

impl<'a> From<r2d2::Error> for ErrorWrapper {
    fn from(e: r2d2::Error) -> Self {
        Self {
            status: Status::InternalServerError,
            name: e.name().unwrap_or("Unknown internal error").to_owned(),
        }
    }
}

impl<'a> From<postgres::Error> for ErrorWrapper {
    fn from(e: postgres::Error) -> Self {
        Self {
            status: Status::InternalServerError,
            name: e.to_string(),
        }
    }
}

impl<'r> Responder<'r> for ErrorWrapper {
    fn respond_to(self, request: &Request<'_>) -> rocket::response::Result<'r> {
        use rocket::response::status::Custom;
        Custom(self.status, Json(self.name)).respond_to(request)
    }
}
