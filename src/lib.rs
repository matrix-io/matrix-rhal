pub mod bus;
pub mod error;

/// The Different types of MATRIX Devices
#[derive(Debug)]
pub enum Device {
    Creator,
    Voice,
}
