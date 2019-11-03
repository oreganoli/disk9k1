use crc32fast::Hasher;
use rocket::Data;
use rocket::http::ContentType;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, RawField,
};
use rocket_multipart_form_data::mime::Mime;

use crate::prelude::*;

#[post("/upload", data = "<data>")]
pub fn upload(
    instance: LockState,
    tera: TeraState,
    content_type: &ContentType,
    data: Data,
) -> Html<String> {
    let mut inst = instance.write().unwrap();

    let mut options = MultipartFormDataOptions::new();
    options
        .allowed_fields
        .push(MultipartFormDataField::raw("file"));
    let mut form_data = MultipartFormData::parse(content_type, data, options).unwrap();
    let x = form_data.raw.remove("file");

    x.map_or_else(|| Html(format!("<p>Upload failed</p>")), |f| match f {
        RawField::Single(field) => {
            let mut ctx = Context::new();
            let mut hasher = Hasher::new();
            hasher.update(&field.raw);
            let hash = hasher.finalize();
            if inst.files.contains_key(&hash) {
                ctx.insert("reason", &format!("A file with the CRC checksum {} already exists.", hash));
                return Html(tera.render("upload_error.html", &ctx).unwrap())
            }
            if field.raw.len() > inst.size_limit {
                let str = format!("<p><b>The file is {} bytes long, which exceeds the limit of {} bytes.</b></p>", field.raw.len(), inst.size_limit);
                ctx.insert("reason", &str);
                return Html(tera.render("upload_error.html", &ctx).unwrap())
            }
            let file = File {
                content_type: content_type.clone(),
                original_name: field.file_name.unwrap_or("file".to_owned()),
                data: field.raw,
            };
            ctx.insert("id", &hash);
            ctx.insert("filename", &file.original_name);
            inst.files.insert(hash, file);
            Html(tera.render("successful_upload.html", &ctx).unwrap())
        },
        _ => {
            let mut ctx = Context::new();
            ctx.insert("reason", "Some other error occurred.");
            Html(tera.render("upload_error.html", &ctx).unwrap())
        }
    })
}
