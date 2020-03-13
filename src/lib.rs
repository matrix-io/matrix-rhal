#![cfg_attr(not(feature = "std"), no_std)]

pub mod bus;
mod error;
mod everloop;
pub mod gpio;
mod sensors;

pub use bus::Bus;
pub use error::Error;
pub use everloop::Everloop;
pub use everloop::Rgbw;
pub use gpio::Gpio;
pub use sensors::Sensors;

#[macro_export]
macro_rules! with_std { ($($i:item)*) => ($(#[cfg(feature = "std")]$i)*) }
#[macro_export]
macro_rules! without_std { ($($i:item)*) => ($(#[cfg(not(feature = "std"))]$i)*) }

/// The Different types of MATRIX Devices
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Device {
    /// MATRIX Creator.
    Creator,
    /// MATRIX Voice.
    Voice,
    /// Placeholder until the device is known.
    Unknown,
}
