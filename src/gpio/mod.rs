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
    /// History of each time a pin's mode was set.
    mode_history: Mutex<u16>,
    /// History of each time a pin's state was set.
    state_history: Mutex<u16>,
    /// History of each time a pin's function was set.
    function_history: Mutex<u16>,
    /// History of each time a pin's prescaler was set.
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
