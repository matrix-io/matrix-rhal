pub mod bus;
use std::sync::Mutex;

/// The Different types of MATRIX Devices
#[derive(Debug)]
pub enum Devices {
    Creator,
    Voice,
}
