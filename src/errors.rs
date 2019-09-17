//! Error handling.

use failure::Fail;

pub type Result<T> = core::result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
    /// Parsing error due to invalid capability name.
    #[fail(display = "invalid capability name: '{}'", _0)]
    InvalidCapName(String),

    /// Syscall error, as `errno(3)`.
    #[fail(display = "syscall failed with '{}'", _0)]
    Sys(errno::Errno),
}
