//! Errors specific to the sounding-wyoming-text-list crate.
use std::fmt::{Display, Formatter, Result};

/// Basic error originating in this crate with a backtrace.
#[derive(Debug)]
pub enum Error {
    /// Used to propagate general errors, such is std::io::Error. These errors are not specific to
    /// this crate.
    InternalError(Box<dyn std::error::Error>),
}

impl Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Error parsing bufkit file.")
    }
}

impl std::error::Error for Error {}
