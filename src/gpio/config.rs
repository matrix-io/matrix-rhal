use super::Gpio;
use crate::error::Error;
use std::fmt;

pub trait PinConfig {
    /// Returns a tuple with a number, binary representation of each pin config, and an FPGA address offset for the config being changed.
    fn update_pin_map(&self, pin: u8, gpio: &Gpio) -> Result<(u16, u16), Error>;
}

/// Specifies if a pin is being used for Output or Input signals.
#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Input = 0,
    Output = 1,
}
impl PinConfig for Mode {
    fn update_pin_map(&self, pin: u8, gpio: &Gpio) -> Result<(u16, u16), Error> {
        let mode = *self as u16;
        let mask = 1 << pin;
        let mut pin_map = gpio.mode_pin_map.lock()?;

        *pin_map = mode << pin | (*pin_map & !mask);

        Ok((*pin_map, 0))
    }
}

/// Specifies if the current state of a pin is `Off`(0) or `On`(1).
#[derive(Copy, Clone)]
pub enum State {
    Off = 0,
    On = 1,
}

impl PinConfig for State {
    fn update_pin_map(&self, pin: u8, gpio: &Gpio) -> Result<(u16, u16), Error> {
        let state = *self as u16;
        let mask = 1 << pin;
        let mut pin_map = gpio.state_pin_map.lock()?;

        *pin_map = state << pin | (*pin_map & !mask);

        Ok((*pin_map, 1))
    }
}

impl fmt::Display for State {
    // Convert State::On and State::Off to 1 and 0 respectively
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl fmt::Debug for State {
    // Convert State::On and State::Off to 1 and 0 respectively
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

/// Specifies which function a pin is using.
#[derive(Debug, Copy, Clone)]
pub enum Function {
    Digital = 0,
    Pwm = 1,
}

impl PinConfig for Function {
    fn update_pin_map(&self, pin: u8, gpio: &Gpio) -> Result<(u16, u16), Error> {
        let function = *self as u16;
        let mask = 1 << pin;
        let mut pin_map = gpio.function_pin_map.lock()?;

        *pin_map = function << pin | (*pin_map & !mask);

        Ok((*pin_map, 2))
    }
}
