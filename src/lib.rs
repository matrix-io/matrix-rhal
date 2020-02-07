mod bus;
pub mod error;
mod sensors;

pub use bus::Bus;
pub use sensors::Sensors;
// pub use sensors::Sensors;

/// The Different types of MATRIX Devices
#[derive(Debug, PartialEq)]
pub enum Device {
    /// MATRIX Creator.
    Creator,
    /// MATRIX Voice.
    Voice,
    /// Placeholder until the device is known.
    Unknown,
}
