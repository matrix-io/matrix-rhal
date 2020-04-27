//! Error handling.

use crate::with_std;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    /// MATRIX Device could not be identified.
    #[fail(display = "Unable to identify MATRIX device.")]
    UnknownDevice,
    /// Could not initialize the MATRIX Bus.
    #[fail(display = "Could not start the MATRIX bus.")]
    UnableToStartBus,
    /// MATRIX Kernel modules have not been installed.
    #[fail(display = "The MATRIX Kernel Modules are not installed.")]
    KernelModulesNotInstalled,
    /// The GPIO pin selected does not exist
    #[fail(display = "The GPIO pin selected does not exist. Valid pins are from 0-15")]
    InvalidGpioPin,

    #[fail(display = "Failed to convert value {} to enum", value)]
    EnumFromIntError { value: u32 },

    /// ESP-IDF call failed.
    #[cfg(not(feature = "std"))]
    #[fail(display = "esp-idf error: {}", error)]
    EspIdf { error: crate::bus::esp::error::EspError },
}

with_std! {
    use nix;
    impl From<nix::Error> for Error {
        fn from(error: nix::Error) -> Self {
            // TODO: add match statement for different nix errors
            // match error {}

            Error::UnableToStartBus
        }
    }
}
