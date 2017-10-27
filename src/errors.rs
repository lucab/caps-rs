//! Error handling.

error_chain!{
    errors {
        /// Parsing error due to invalid capability name.
        InvalidCapName(name: String) {
            description("invalid capability name")
            display("invalid capability name: '{}'", name)
        }
    }
}
