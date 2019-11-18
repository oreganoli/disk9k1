use crate::prelude::*;

#[derive(Debug, Serialize)]
pub enum InstanceError {
    NameEmpty,
    NegativeSizeLimit,
}
