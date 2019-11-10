use std::str::FromStr;

use crc32fast::Hasher;
use mime_sniffer::MimeTypeSniffer;
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
    let inst_data = inst.ins_repo.get().unwrap().unwrap();
    let mut ctx = Context::new();
    let data = data::file_field_from_form(content_type.clone(), data, inst_data.size_limit as u64);
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
                return tera.html("PAGE_upload_error.html", &ctx);
            }
            let mime = &raw.raw.sniff_mime_type();
            use rocket::http::{ContentType, MediaType};
            let content = ContentType(
                MediaType::from_str(mime.unwrap_or("application/octet-stream"))
                    .unwrap_or(MediaType::Binary),
            );
            let file = data::file_from_raw(&content, &raw);
            ctx.insert("id", &hash);
            let url = rocket::http::uri::Uri::percent_encode(&file.original_name);
            ctx.insert("filename", &url);
            inst.files.insert(hash, file);
            tera.html("PAGE_successful_upload.html", &ctx)
        }
        Err(e) => {
            let reason = match e {
                FFE::NoData => "No file data was provided".to_owned(),
                FFE::BadForm => "The upload form was invalid.".to_owned(),
                FFE::TooLarge => format!(
                    "The file exceeded the size limit of {} MiB.",
                    mebibytes(inst_data.size_limit as u64)
                ),
                FFE::Other => "Some unhandled upload error occurred.".to_owned(),
            };
            ctx.insert("reason", &reason);
            tera.html("PAGE_upload_error.html", &ctx)
        }
    }
}
