mod bus;
mod error;
mod everloop;
pub mod gpio;
pub mod microphone;
mod sensors;

pub use bus::Bus;
pub use error::Error;
pub use everloop::Everloop;
pub use everloop::Rgbw;
pub use gpio::Gpio;
pub use sensors::Sensors;

/// The Different types of MATRIX Devices
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Device {
    /// MATRIX Creator.
    Creator,
    /// MATRIX Voice.
    Voice,
    /// Placeholder until the device is known.
    Unknown,
}
