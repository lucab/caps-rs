//! Error handling.

/// Library errors.
#[derive(Debug)]
pub struct CapsError(pub(crate) String);

impl From<&str> for CapsError {
    fn from(arg: &str) -> Self {
        Self(arg.to_string())
    }
}

impl From<String> for CapsError {
    fn from(arg: String) -> Self {
        Self(arg)
    }
}

impl std::error::Error for CapsError {}

impl std::fmt::Display for CapsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "caps error: {}", self.0)
    }
}
