pub mod bus;
pub mod error;

/// The Different types of MATRIX Devices
#[derive(Debug)]
pub enum Device {
    /// MATRIX Creator.
    Creator,
    /// MATRIX Voice.
    Voice,
    /// Placeholder until device is known.
    Unknown,
}
