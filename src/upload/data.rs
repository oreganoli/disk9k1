use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError as MFDE, MultipartFormDataField,
    MultipartFormDataOptions, RawField, SingleRawField,
};

use crate::prelude::*;

fn single_file_options<'a>(size_limit: u64) -> MultipartFormDataOptions<'a> {
    let mut opts = MultipartFormDataOptions::new();
    let mut raw = MultipartFormDataField::raw("file");
    raw.size_limit = size_limit;
    opts.allowed_fields.push(raw);
    opts
}

pub fn file_from_raw(content_type: &ContentType, field: &SingleRawField) -> File {
    File {
        content_type: content_type.clone(),
        original_name: field
            .file_name
            .as_ref()
            .map_or(String::from("file"), |s| s.clone()),
        data: field.raw.clone(),
    }
}

fn single_raw_field(form_data: MultipartFormData) -> Result<SingleRawField, FileFormError> {
    match form_data.raw.get("file") {
        Some(f) => match f {
            RawField::Single(single) => Ok(SingleRawField {
                content_type: single.content_type.clone(),
                file_name: single.file_name.clone(),
                raw: single.raw.clone(),
            }),
            _ => Err(FileFormError::BadForm),
        },
        _ => Err(FileFormError::NoData),
    }
}

pub fn file_field_from_form(
    content_type: ContentType,
    data: Data,
    size_limit: u64,
) -> Result<SingleRawField, FileFormError> {
    let form_data: MultipartFormData;
    match MultipartFormData::parse(&content_type.clone(), data, single_file_options(size_limit)) {
        Ok(data) => form_data = data,
        Err(MFDE::DataTooLargeError(_)) => return Err(FileFormError::TooLarge),
        _ => return Err(FileFormError::Other),
    }
    single_raw_field(form_data)
}

pub enum FileFormError {
    TooLarge,
    NoData,
    BadForm,
    Other,
}
