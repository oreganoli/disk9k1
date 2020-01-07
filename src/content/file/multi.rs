use rocket::data::{FromDataSimple, Outcome};
use rocket::http::{ContentType, MediaType};
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    mime, FileField, MultipartFormData, MultipartFormDataError, MultipartFormDataField,
    MultipartFormDataOptions, RawField, SingleFileField, SingleTextField, TextField,
};

use crate::prelude::*;

use super::{FileError, NewFile};

impl FromDataSimple for NewFile {
    type Error = ErrorWrapper;
    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
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
            .push(MultipartFormDataField::raw("data").size_limit(1024 * 1024 * 1024));
        let form = match MultipartFormData::parse(&ContentType::FormData, data, options) {
            Ok(m) => m,
            _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
        };
        let filename = match form.texts.get("filename") {
            Some(field) => match field {
                TextField::Single(str) => str.text.clone(),
                _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
            },
            _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
        };
        if filename.is_empty() {
            return Failure((Status::BadRequest, FileError::NoFileName.into()));
        }
        let public = match form.texts.get("public") {
            Some(field) => match field {
                TextField::Single(str) => str.text == "true",
                _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
            },
            _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
        };
        let directory = match form.texts.get("directory") {
            Some(field) => match field {
                TextField::Single(str) => match str::parse::<i32>(&str.text) {
                    Ok(0) => None,
                    Ok(id) => Some(id),
                    _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
                },
                _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
            },
            _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
        };
        let data = match form.raw.get("data") {
            Some(field) => match field {
                RawField::Single(single) => single.raw.clone(),
                _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
            },
            _ => return Failure((Status::BadRequest, FileError::ImproperForm.into())),
        };
        Success(NewFile {
            filename,
            public,
            directory,
            data,
        })
    }
}
