/////////////////////////////////////////
// TODO: CLEAN THIS ALL UP
////////////////////////////////////////


//! Error handling.
use std::{error::Error as StdError, fmt};

/// An error that can occur in the server.
#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    /// An error returned by the custom `unimplemented!` macro.

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Any(error) => error.fmt(f),
            Error::Unimplemented {
                file,
                line,
                message,
            } => {
                write!(f, "[{}:{}] unimplemented", file, line)?;

                if let Some(message) = message {
                    write!(f, ": \"{}\"", message)?;
                }

                Ok(())
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Any(error) => Some(&**error),
            Error::Unimplemented { .. } => None,
        }
    }
}
