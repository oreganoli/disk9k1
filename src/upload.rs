use crc32fast::Hasher;
use rocket::http::ContentType;
use rocket::Data;

use crate::prelude::*;

mod data;

#[post("/upload", data = "<data>")]
pub fn upload(
    instance: LockState,
    tera: TeraState,
    content_type: &ContentType,
    data: Data,
) -> Page {
    let mut inst = instance.write().unwrap();
    let mut ctx = Context::new();
    let data = data::file_field_from_form(content_type.clone(), data, inst.size_limit as u64);
    use data::FileFormError as FFE;
    match data {
        Ok(raw) => {
            let mut hasher = Hasher::new();
            hasher.update(&raw.raw);
            let hash = hasher.finalize();
            if inst.files.contains_key(&hash) {
                ctx.insert(
                    "reason",
                    &format!("A file with the CRC checksum {} already exists.", hash),
                );
                return tera.html("upload_error.html", &ctx);
            }
            let file = data::file_from_raw(content_type, &raw);
            ctx.insert("id", &hash);
            ctx.insert("filename", &file.original_name);
            inst.files.insert(hash, file);
            tera.html("successful_upload.html", &ctx)
        }
        Err(e) => {
            let reason = match e {
                FFE::NoData => "No file data was provided".to_owned(),
                FFE::BadForm => "The upload form was invalid.".to_owned(),
                FFE::TooLarge => format!(
                    "The file exceeded the size limit of {}B.",
                    mebibytes(inst.size_limit as u64)
                ),
                FFE::Other => "Some unhandled upload error occurred.".to_owned(),
            };
            ctx.insert("reason", &reason);
            tera.html("upload_error.html", &ctx)
        }
    }
}
