mod bus;
pub mod error;
mod everloop;
mod sensors;

pub use bus::Bus;
pub use everloop::Everloop;
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
