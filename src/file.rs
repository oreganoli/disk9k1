use std::io::Cursor;

use rocket::http::ContentType;
use rocket::response::{Content, Stream};

use crate::prelude::*;

pub struct File {
    pub original_name: String,
    pub content_type: ContentType,
    pub data: Vec<u8>,
}

#[get("/metadata/<id>")]
pub fn file_info(instance: LockState, tera: TeraState, id: u32) -> Option<Page> {
    let inst = instance.read().unwrap();
    inst.files.get(&id).and_then(|file| {
        let mut ctx = Context::new();
        ctx.insert("id", &id);
        ctx.insert("original_name", &file.original_name);
        Some(tera.html("file_info.html", &ctx))
    })
}

type FileGet = Option<Content<Stream<Cursor<Vec<u8>>>>>;

fn file(instance: LockState, id: u32) -> FileGet {
    let inst = instance.read().unwrap();
    inst.files.get(&id).and_then(|file| {
        Some(Content(
            file.content_type.clone(),
            Stream::from(Cursor::new(file.data.clone())),
        ))
    })
}

#[get("/file/<id>")]
pub fn get_file(instance: LockState, id: u32) -> FileGet {
    file(instance, id)
}

#[get("/file/<id>/<_name>")]
pub fn get_file_named(instance: LockState, id: u32, _name: Option<String>) -> FileGet {
    file(instance, id)
}
