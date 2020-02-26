use crate::bus::memory_map::*;
use crate::Bus;
pub mod setting;
pub use setting::*;

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

    // TODO: fix
    /// Shortener to set pin configurations. Value is directly included in the bus' write buffer.
    fn pin_set(&self, pin: u8, address_offset: u16, value: u32) {
        if pin > 15 {
            panic!("The MATRIX Voice/Creator GPIO pins are from 0-15");
        }

        // bit operation to encode pin mode
        let mask = 1 << pin;
        let value = value << pin | (0x0 & !mask);

        // create write buffer
        let mut data: [u32; 3] = [0; 3];

        data[0] = (fpga_address::GPIO + address_offset) as u32;
        data[1] = value;

        self.bus
            .write(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(&mut data) });
    }

    // TODO: fix
    /// Configure pin to be Output or Input.
    pub fn set_mode(&self, pin: u8, mode: Mode) {
        self.pin_set(
            pin,
            0,
            match mode {
                Mode::Input => 0,
                Mode::Output => 1,
            },
        );
    }

    // TODO: fix
    pub fn set_value(&self, pin: u8, value: u32) {
        self.pin_set(pin, 1, value);
    }

    // TODO: fix
    /// Configure pin to be Digital or PWM.
    pub fn set_function(&self, pin: u8, function: Function) {
        self.pin_set(
            pin,
            2,
            match function {
                Function::Digital => 0,
                Function::Pwm => 1,
            },
        );
    }

    /// Returns the digital value of a MATRIX GPIO pin (0-15).
    pub fn get_value(&self, pin: u8) -> u8 {
        if pin > 15 {
            panic!("The MATRIX Voice/Creator GPIO pins are from 0-15");
        }

        // create read buffer
        let mut data: [u32; 3] = [0; 3];

        data[0] = (fpga_address::GPIO + 1) as u32;
        data[1] = 2; // 2 bytes needed (8*2 = 16 pins)

        // populate buffer
        // the buffer will be passed a value that contains the state of each GPIO pin
        self.bus
            .read(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(&mut data) });

        // bit operation to extract the current pin state
        let mask = 0x1 << pin;
        ((data[2] & mask) >> pin) as u8
    }

    pub fn get_values(&self) -> [u8; 16] {
        // create read buffer
        let mut data: [u32; 3] = [0; 3];

        data[0] = (fpga_address::GPIO + 1) as u32;
        data[1] = 2; // 2 bytes needed (8*2 = 16 pins)

        // populate buffer
        // the buffer will be passed a value that contains the state of each GPIO pin
        self.bus
            .read(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(&mut data) });

        // bit operation to extract each pin state (0-15)
        let mut pins = [0; 16];
        for i in 0..16 {
            let mask = 0x1 << i;
            pins[i] = ((data[2] & mask) >> i) as u8
        }

        pins
    }
}
