use std::{borrow::Cow, convert::Infallible};

/// widevine error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// error encoding/decoding protobuf
    #[error("protobuf: {0}")]
    Protobuf(#[from] protobuf::Error),
    /// file i/o error
    #[error("i/o: {0}")]
    Io(#[from] std::io::Error),
    #[error("rsa: {0}")]
    /// rsa error
    Rsa(#[from] rsa::Error),
    /// received invalid input
    #[error("invalid input: {0}")]
    InvalidInput(Cow<'static, str>),
    /// received invalid license
    #[error("invalid license: {0}")]
    InvalidLicense(Cow<'static, str>),
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}