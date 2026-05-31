use arrow::error::ArrowError;
use std::fmt;

#[derive(Debug)]
pub enum JetmetricsError {
    Arrow(ArrowError),
    InvalidInput(String),
    NumericalError(String),
}

impl fmt::Display for JetmetricsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arrow(e) => write!(f, "Arrow error: {e}"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            Self::NumericalError(msg) => write!(f, "Numerical error: {msg}"),
        }
    }
}

impl std::error::Error for JetmetricsError {}

impl From<ArrowError> for JetmetricsError {
    fn from(e: ArrowError) -> Self {
        Self::Arrow(e)
    }
}

pub type Result<T> = std::result::Result<T, JetmetricsError>;
