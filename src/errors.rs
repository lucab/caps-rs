//! Error handling.

use errno;

error_chain!{
    errors {
        /// Parsing error due to invalid capability name.
        InvalidCapName(name: String) {
            description("invalid capability name")
            display("invalid capability name: '{}'", name)
        }
        /// Syscall error, as `errno(3)`.
        Sys(errno: errno::Errno) {
            description("syscall failed")
            display("{}", errno)
        }
    }
}
