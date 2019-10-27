use rocket::Data;
use rocket::http::ContentType;
use rocket_multipart_form_data::{FileField, mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};

use crate::prelude::*;

#[post("/upload", data = "<data>")]
pub fn upload(content_type: &ContentType, data: Data) -> Html<String> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::file("file"));
    let mut form_data = MultipartFormData::parse(content_type, data, options).unwrap();
    let x = form_data.files.remove("file");
    match x {
        Some(f) => {
            match f {
                FileField::Single(g) => {
                    Html(format!("<p>Filename: {:?}</p><p>Content type: {:?}</p><p>Path: {:?}</p>", g.file_name, g.content_type, g.path))
                }
                _ => Html(format!("<p>Upload failed</p>"))
            }
        }
        _ => Html(format!("<p>Upload failed</p>"))
    }
}