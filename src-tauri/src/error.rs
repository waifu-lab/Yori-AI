use bincode::ErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Invalid path provided")]
    InvalidPath,
    #[error("File or directory not found")]
    NotFound,
    #[error("Permission denied")]
    PermissionDenied,
    #[error(transparent)]
    IoError(std::io::Error),
}

#[derive(Error, Debug)]
pub enum SerializeError {
    #[error(transparent)]
    SerializationFailed(Box<ErrorKind>),
    #[error(transparent)]
    DeserializationFailed(Box<ErrorKind>),
}