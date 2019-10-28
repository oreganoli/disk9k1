use rocket::http::ContentType;

use crate::prelude::*;

pub struct File {
    pub original_name: String,
    pub content_type: ContentType,
    pub data: Vec<u8>,
}