use crate::message::{OutputMessage, RequestId};
use crate::value;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("DBus error: {0}")]
    DBusError(#[from] zbus::Error),
    #[error("Server error: {0}")]
    ServerError(#[from] axum::Error),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("DBus message format error: {0}")]
    DBusFormatError(#[from] zvariant::Error),
    #[error("DBus value error: {0}")]
    DBusValueError(#[from] value::Error),
}

impl Error {
    fn error_type(&self) -> ErrorType {
        match self {
            Error::DBusError(_) => ErrorType::DBusError,
            Error::ServerError(_) => ErrorType::ServerError,
            Error::UnsupportedFormat(_) => ErrorType::UnsupportedFormat,
            Error::JsonError(_) => ErrorType::JsonError,
            Error::DBusFormatError(_) => ErrorType::DBusFormatError,
            Error::DBusValueError(_) => ErrorType::DBusValueError,
        }
    }
}

#[derive(Error, Debug)]
pub struct RequestError {
    request_id: Option<RequestId>,
    #[source]
    error: Error,
}

impl RequestError {
    pub(crate) fn new(request_id: Option<RequestId>, error: impl Into<Error>) -> Self {
        Self {
            request_id,
            error: error.into(),
        }
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, f)
    }
}

impl From<RequestError> for Error {
    fn from(value: RequestError) -> Self {
        value.error
    }
}

impl From<Error> for RequestError {
    fn from(error: Error) -> Self {
        RequestError {
            error,
            request_id: None,
        }
    }
}

impl From<Error> for OutputMessage {
    fn from(error: Error) -> Self {
        RequestError::new(None, error).into()
    }
}

impl From<RequestError> for OutputMessage {
    fn from(RequestError { request_id, error }: RequestError) -> Self {
        OutputMessage::Error {
            request_id,
            error_type: error.error_type(),
            message: error.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ErrorType {
    DBusError,
    ServerError,
    UnsupportedFormat,
    JsonError,
    DBusFormatError,
    DBusValueError,
}
