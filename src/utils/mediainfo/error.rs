use std::ffi::NulError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MediaInfoError {
    #[error("could not open file - file not found")]
    OpenFailed,

    #[error("null error")]
    NullError(#[from] NulError),

    #[error("utf8 error")]
    Utf8Error(std::str::Utf8Error),
}
