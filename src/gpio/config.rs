use super::Gpio;
use crate::bus::MatrixBus;
use crate::error::Error;

pub trait PinConfig {
    /// Returns a tuple of a number (binary representation of each pin config) and an FPGA address offset for the config being changed.
    fn update_pin_map<B: MatrixBus>(&self, pin: u8, gpio: &Gpio<B>) -> Result<(u16, u16), Error>;
}

/// Represents a pin being used for `Output` or `Input`.
#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Input = 0,
    Output = 1,
}

impl PinConfig for Mode {
    fn update_pin_map<B: MatrixBus>(&self, pin: u8, gpio: &Gpio<B>) -> Result<(u16, u16), Error> {
        let pin_map = &mut *gpio.mode_pin_map.lock()?;
        Ok((set_pin_config(pin, *self as u16, pin_map), 0))
    }
}

/// Represents a pin being `On` or `Off`.
#[derive(Debug, Copy, Clone)]
pub enum State {
    Off = 0,
    On = 1,
}

impl PinConfig for State {
    fn update_pin_map<B: MatrixBus>(&self, pin: u8, gpio: &Gpio<B>) -> Result<(u16, u16), Error> {
        let pin_map = &mut *gpio.state_pin_map.lock()?;
        Ok((set_pin_config(pin, *self as u16, pin_map), 1))
    }
}

/// Represents a pin being used for `Digital` or `Pwm`.
#[derive(Debug, Copy, Clone)]
pub enum Function {
    Digital = 0,
    Pwm = 1,
}

impl PinConfig for Function {
    fn update_pin_map<B: MatrixBus>(&self, pin: u8, gpio: &Gpio<B>) -> Result<(u16, u16), Error> {
        let pin_map = &mut *gpio.function_pin_map.lock()?;
        Ok((set_pin_config(pin, *self as u16, pin_map), 2))
    }
}

/// Flips the desired bit(pin) in a configuration's pin map.
///
/// # Code Explanation
/// ```
///     let pin = 15;
///     let config = 0;
///     let mut pin_map = 32771; // ->10000000000000011
///     let mask = 1 << pin; // -> 1000000000000000
///
///     let config = config << pin; // -> 0000000000000000
///     let configured_map = pin_map & !mask; // -> 0000000000000011
///     
///     pin_map = config | configured_map; // -> 0000000000000011
/// ```
fn set_pin_config(pin: u8, config: u16, pin_map: &u16) -> u16 {
    let mask = 1 << pin;
    config << pin | (pin_map & !mask)
}
