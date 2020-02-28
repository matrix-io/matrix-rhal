use crate::bus::memory_map::*;
use crate::Bus;
pub mod config;
pub use config::*;
mod get;
mod set;

// TODO: add the following constants
// uint16_t mode_;
// uint16_t value_;
// uint16_t function_;
// uint16_t prescaler_;

/// Controls the GPIO pins on a MATRIX device.
pub struct Gpio<'a> {
    bus: &'a Bus,
}

impl<'a> Gpio<'a> {
    /// Returns an instance of GPIO.
    pub fn new(bus: &'a Bus) -> Gpio {
        Gpio { bus }
    }
}
