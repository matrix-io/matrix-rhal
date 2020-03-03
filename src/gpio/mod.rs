use crate::Bus;
pub mod config;
pub use config::*;
use std::sync::Mutex;
mod get;
mod set;

/// Controls the GPIO pins on a MATRIX device.
#[derive(Debug)]
pub struct Gpio<'a> {
    bus: &'a Bus,
    /// Current setting of each pin's mode (binary representation).
    mode_history: Mutex<u16>,
    /// Current setting of each pin's state (binary representation).
    state_history: Mutex<u16>,
    /// Current setting of each pin's function (binary representation).
    function_history: Mutex<u16>,
    /// Current setting of each pin's prescaler (binary representation).
    prescaler_history: Mutex<u16>,
}

impl<'a> Gpio<'a> {
    /// Returns an instance of GPIO.
    pub fn new(bus: &'a Bus) -> Gpio {
        Gpio {
            bus,
            mode_history: Mutex::new(0x0),
            state_history: Mutex::new(0x0),
            function_history: Mutex::new(0x0),
            prescaler_history: Mutex::new(0x0),
        }
    }
}
