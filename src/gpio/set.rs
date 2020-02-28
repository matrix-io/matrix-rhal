use super::config::*;
use super::Gpio;
use crate::bus::memory_map::*;

impl<'a> Gpio<'a> {
    /// Configure a pin to be used for Digital or PWM functions.
    pub fn set_function(&self, pin: u8, function: Function) {
        if pin > 15 {
            panic!("The MATRIX Voice/Creator GPIO pins are from 0-15");
        }

        let function = match function {
            Function::Digital => 0,
            Function::Pwm => 1,
        };

        // bit operation to encode value
        let mask = 1 << pin;
        let function = function << pin | (0x0 & !mask);

        self.pin_set(function, 2);
    }

    /// Configure pin to be Output or Input.
    pub fn set_mode(&self, pin: u8, mode: Mode) {
        if pin > 15 {
            panic!("The MATRIX Voice/Creator GPIO pins are from 0-15");
        }

        let mode = match mode {
            Mode::Input => 0,
            Mode::Output => 1,
        };

        // bit operation to encode value
        let mask = 1 << pin;
        let mode = mode << pin | (0x0 & !mask);

        self.pin_set(mode, 0);
    }

    /// Configure a pin to be in an ON or OFF state.
    pub fn set_value(&self, pin: u8, state: State) {
        if pin > 15 {
            panic!("The MATRIX Voice/Creator GPIO pins are from 0-15");
        }

        let state = match state {
            State::On => 1,
            State::Off => 0,
        };

        // bit operation to encode value
        let mask = 1 << pin;
        let state = state << pin | (0x0 & !mask);

        self.pin_set(state, 1);
    }

    /// Shortener to set pin configurations. Value is directly included in the bus' write buffer.
    fn pin_set(&self, value: u32, address_offset: u16) {
        // create and populate write buffer
        let mut buffer: [u32; 3] = [0; 3];
        buffer[0] = (fpga_address::GPIO + address_offset) as u32; // address to write to
        buffer[1] = 2; // byte length of value  // TODO: ask about what the length is tied to.
        buffer[2] = value;

        self.bus
            .write(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(&mut buffer) });
    }
}
