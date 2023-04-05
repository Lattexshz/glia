use std::{error, fmt};
use crate::error::_GLDKError::{Custom, Simple};

#[derive(Clone)]
pub enum ErrorKind {
    Unexpected(String)
}

impl ErrorKind {
    pub fn description(&self) -> String {
        match self {
            ErrorKind::Unexpected(detail) => "Unexpected error! Detailed description: ".to_owned() + detail
        }
    }
}

pub struct GLDKError {
    _error: _GLDKError,
}

impl GLDKError {
    fn kind(&self) -> ErrorKind {
        match &self._error {
            Simple(s) => s.clone(),
            Custom(c) => c.0.clone(),
        }
    }
}

impl GLDKError {
    // 他のエラー型を受け取れるようにコンストラクタを作っておく
    pub(crate) fn new<E>(kind: ErrorKind, error: E) -> Self
        where
            E: Into<Box<dyn error::Error + Send + Sync>>
    {
        Self { _error: _GLDKError::Custom((kind, error.into())) }
    }

    pub(crate) fn new_unexpected(desc: String) -> Self {
        Self { _error: _GLDKError::Simple(ErrorKind::Unexpected(desc)) }
    }
}

impl fmt::Debug for GLDKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for GLDKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.kind().description())
    }
}

impl error::Error for GLDKError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self._error {
            _GLDKError::Simple(_) => None,
            _GLDKError::Custom(c) => c.1.source()
        }
    }
}

enum _GLDKError {
    Simple( ErrorKind ),
    Custom( (ErrorKind, Box<dyn error::Error + Send + Sync>) ),
}

