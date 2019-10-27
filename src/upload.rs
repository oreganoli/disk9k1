use rocket::Data;
use rocket::http::ContentType;
use rocket_multipart_form_data::{FileField, mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, RawField};

use crate::prelude::*;

#[post("/upload", data = "<data>")]
pub fn upload(content_type: &ContentType, data: Data) -> Html<String> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::raw("file"));
    let mut form_data = MultipartFormData::parse(content_type, data, options).unwrap();
    let x = form_data.raw.remove("file");
    match x {
        Some(f) => {
            match f {
                RawField::Single(f) => {
                    Html(
                        format!("<p>Filename: {:?}</p><p>Content type: {:?}</p><p>Data:</p><pre>{}</pre>",
                                f.file_name,
                                f.content_type,
                                std::str::from_utf8(&f.raw).unwrap()
                        )
                    )
                }
                _ => Html(format!("<p>Upload failed</p>"))
            }
        }
        _ => Html(format!("<p>Upload failed</p>"))
    }
}