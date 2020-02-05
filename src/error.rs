//! Error handling.
use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    /// MATRIX Device could not be identified.
    UnknownDevice,
    /// Could not initialize the MATRIX Bus.
    UnableToStartBus,
    /// The mutex for the Bus could not be grabbed.
    PoisonedBusMutex,
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownDevice => write!(f, "Unable to identify MATRIX device."),
            Error::UnableToStartBus => write!(f, "Could not start the MATRIX bus."),
            Error::PoisonedBusMutex => write!(f, "Mutex for MATRIX bus is unreachable."),
            _ => write!(f, "TODO: ADD ERROR DESCRIPTION!"),
        }
    }
}

use nix;
impl From<nix::Error> for Error {
    fn from(error: nix::Error) -> Self {
        // TODO: add match statement for different nix errors
        // match error {}

        Error::UnableToStartBus
    }
}

use std::sync::MutexGuard;
use std::sync::PoisonError;
impl From<PoisonError<MutexGuard<'_, ()>>> for Error {
    fn from(_: PoisonError<MutexGuard<()>>) -> Self {
        Error::PoisonedBusMutex
    }
}
