use rocket::http::ContentType;

use crate::prelude::*;

pub struct File<'a> {
    pub original_name: String,
    pub content_type: ContentType,
    pub data: Box<&'a [u8]>,
}