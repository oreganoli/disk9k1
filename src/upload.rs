use crc32fast::Hasher;
use rocket::Data;
use rocket::http::ContentType;
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions, RawField, SingleRawField};

use crate::prelude::*;

#[post("/upload", data = "<data>")]
pub fn upload(
    instance: LockState,
    tera: TeraState,
    content_type: &ContentType,
    data: Data,
) -> Html<String> {
    let mut inst = instance.write().unwrap();
    let mut ctx = Context::new();
    let mut options = MultipartFormDataOptions::new();
    let mut raw = MultipartFormDataField::raw("file");
    let mut hasher = Hasher::new();
    raw.size_limit = inst.size_limit as u64;
    options
        .allowed_fields
        .push(raw);
    let form_data: MultipartFormData;
    match MultipartFormData::parse(content_type, data, options) {
        Ok(data) => form_data = data,
        Err(e) => {
            match e {
                MultipartFormDataError::DataTooLargeError(_) => {
                    ctx.insert("reason", &format!("The file exceeded the size limit of {}B.", inst.size_limit));
                    return Html(tera.render("upload_error.html", &ctx).unwrap())
                },
                _ => {
                    ctx.insert("reason", "Some unhandled error occurred.");
                    return Html(tera.render("upload_error.html", &ctx).unwrap())
                }
            }
        }
    }
    let field_option = form_data.raw.get("file");
    let actual_field: &SingleRawField;
    match field_option {
        None => {
            ctx.insert("reason", "No file data was provided.");
            return Html(tera.render("upload_error.html", &ctx).unwrap())
        },
        Some(field_type) => {
            match field_type {
                RawField::Single(field) => actual_field = field,
                _ => {
                    ctx.insert("reason", "More than one file field was given.");
                    return Html(tera.render("upload_error.html", &ctx).unwrap())
                }
            }
        }
    }
    hasher.update(&actual_field.raw);
    let hash = hasher.finalize();
    if inst.files.contains_key(&hash) {
        ctx.insert("reason", &format!("A file with the CRC checksum {} already exists.", hash));
        return Html(tera.render("upload_error.html", &ctx).unwrap())
    }
    let field_len = actual_field.raw.len();
    if field_len > inst.size_limit {
        let str = format!("The file, sized {}B, exceeded the size limit of {}B.", field_len, inst.size_limit);
        ctx.insert("reason", &str);
        return Html(tera.render("upload_error.html", &ctx).unwrap())
    }
    let file = File {
        content_type: content_type.clone(),
        original_name: actual_field.file_name.as_ref().map_or(String::from("file"), |s| s.clone()),
        data: actual_field.raw.clone(),
    };
    ctx.insert("id", &hash);
    ctx.insert("filename", &file.original_name);
    inst.files.insert(hash, file);
    Html(tera.render("successful_upload.html", &ctx).unwrap())

}
