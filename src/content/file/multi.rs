use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, RawField, TextField,
};

use crate::prelude::*;

use super::{FileError, NewFile};

pub fn from_form(content_type: ContentType, data: Data, size: usize) -> AppResult<NewFile> {
    let mut options = MultipartFormDataOptions::new();
    options
        .allowed_fields
        .push(MultipartFormDataField::text("filename"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("public"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("directory"));
    options
        .allowed_fields
        .push(MultipartFormDataField::raw("data").size_limit(size as u64));
    let form = match MultipartFormData::parse(&content_type, data, options) {
        Ok(m) => m,
        Err(e) => {
            dbg!(e);
            return Err(FileError::ImproperForm.into());
        }
    };
    eprintln!("Parsed form.");
    let filename = match form.texts.get("filename") {
        Some(field) => match field {
            TextField::Single(str) => str.text.clone(),
            _ => return Err(FileError::ImproperForm.into()),
        },
        _ => return Err(FileError::ImproperForm.into()),
    };
    eprintln!("Got file name.");
    if filename.is_empty() {
        return Err(FileError::NoFileName.into());
    }
    let public = match form.texts.get("public") {
        Some(field) => match field {
            TextField::Single(str) => str.text == "true",
            _ => return Err(FileError::ImproperForm.into()),
        },
        _ => return Err(FileError::ImproperForm.into()),
    };
    eprintln!("Got publicity.");
    let directory = match form.texts.get("directory") {
        Some(field) => match field {
            TextField::Single(str) => match str::parse::<i32>(&str.text) {
                Ok(0) => None,
                Ok(id) => Some(id),
                _ => return Err(FileError::ImproperForm.into()),
            },
            _ => return Err(FileError::ImproperForm.into()),
        },
        _ => return Err(FileError::ImproperForm.into()),
    };
    eprintln!("Got directory.");
    let data = match form.raw.get("data") {
        Some(field) => match field {
            RawField::Single(single) => single.raw.clone(),
            _ => return Err(FileError::ImproperForm.into()),
        },
        _ => return Err(FileError::ImproperForm.into()),
    };
    if data.len() > size {
        return Err(FileError::TooBig.into());
    } else if data.is_empty() {
        return Err(FileError::ImproperForm.into());
    }
    eprintln!("Got data.");
    eprintln!(
        "FILENAME: {:?}, PUBLICITY: {:?}, DIRECTORY: {:?}, DATA SIZE: {}",
        filename,
        public,
        directory,
        data.len()
    );
    Ok(NewFile {
        filename,
        public,
        directory,
        data,
    })
}
