use std::error::Error as StdError;
use hyper::StatusCode;
use std::fmt;

/// The Errors that may occur when processing a `Request`.
pub struct Error {
    inner: Box<Inner>,
}

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

struct Inner {
    kind: Kind,
    description: String,
    source: Option<BoxError>,
}

impl Error {
    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> Error
        where
            E: Into<BoxError>,
    {
        Error {
            inner: Box::new(Inner {
                kind,
                source: source.map(Into::into),
                description: "".to_string(),
            }),
        }
    }

    /// Returns the status code, if the error was generated from a response.
    pub fn status(&self) -> Option<StatusCode> {
        match self.inner.kind.clone() {
            Kind::Status(code) => Some(code),
            Kind::ErrorV1(code, _0) => Some(code),
            Kind::ErrorV2(code, _0, _1) => Some(code),
            _ => None,
        }
    }

    pub(crate) fn with_prefix<E: std::fmt::Display>(mut self, prefix: E) -> Error {
        self.inner.description = format!("{}{}", prefix, self.inner.description);
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner.description)?;
        match self.inner.kind.clone() {
            Kind::Text(ref text) => {
                write!(f, "{}", text)?;
            }
            Kind::Status(ref code) => {
                let prefix = if code.is_client_error() {
                    "HTTP status client error"
                } else {
                    "HTTP status server error"
                };
                write!(f, "{} ({})", prefix, code)?;
            }
            Kind::ErrorV1(code, error) => {
                let prefix = if code.is_client_error() {
                    "HTTP status client error"
                } else {
                    "HTTP status server error"
                };
                write!(f, "{} ({}) - {}", prefix, code, error)?;
            }
            Kind::ErrorV2(code, error, error_code) => {
                let prefix = if code.is_client_error() {
                    "HTTP status client error"
                } else {
                    "HTTP status server error"
                };
                write!(f, "{} ({}) - {} ({})", prefix, code, error, error_code)?;
            }
        };

        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("reqwest::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}


impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Kind {
    Text(String),
    Status(StatusCode),
    ErrorV1(StatusCode, String),
    ErrorV2(StatusCode, String, String),
}

pub(crate) fn text_error(message: String) -> Error {
    Error::new(Kind::Text(message), None::<Error>)
}

pub(crate) fn text_error_with_inner<E: Into<BoxError>>(message: String, e: E) -> Error {
    Error::new(Kind::Text(message), Some(e))
}

pub(crate) fn status_code(status: StatusCode) -> Error {
    Error::new(Kind::Status(status), None::<Error>)
}

pub(crate) fn v1_error(status: StatusCode, error: String) -> Error {
    Error::new(Kind::ErrorV1(status, error), None::<Error>)
}

pub(crate) fn v2_error(status: StatusCode, error: String, error_code: String) -> Error {
    Error::new(Kind::ErrorV2(status, error, error_code), None::<Error>)
}

