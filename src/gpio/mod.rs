use crate::Bus;
use crate::Error;
pub mod bank;
pub mod config;
pub use bank::*;
pub use config::*;
use std::sync::Mutex;
mod get;
mod set;

/// Controls the GPIO pins on a MATRIX device.
#[derive(Debug)]
pub struct Gpio<'a> {
    bus: &'a Bus,
    /// Current setting of each pin's mode (binary representation).
    mode_pin_map: Mutex<u16>,
    /// Current setting of each pin's state (binary representation).
    state_pin_map: Mutex<u16>,
    /// Current setting of each pin's function (binary representation).
    function_pin_map: Mutex<u16>,
    /// Current setting of each bank's prescaler (binary representation).
    prescaler_bank_map: Mutex<u16>,
    /// Current state of each GPIO Bank.
    banks: Mutex<Vec<Bank<'a>>>,
}

impl<'a> Gpio<'a> {
    /// Returns an instance of GPIO.
    pub fn new(bus: &'a Bus) -> Gpio {
        let banks = vec![Bank::new(&bus).clone(); 4];

        Gpio {
            bus,
            mode_pin_map: Mutex::new(0x0),
            state_pin_map: Mutex::new(0x0),
            function_pin_map: Mutex::new(0x0),
            prescaler_bank_map: Mutex::new(0x0),
            banks: Mutex::new(banks),
        }
    }

    /// A simple check to make sure a selected pin Exists
    fn is_pin_valid(pin: u8) -> Result<(), Error> {
        if pin > 15 {
            return Err(Error::InvalidGpioPin);
        }

        Ok(())
    }
}
