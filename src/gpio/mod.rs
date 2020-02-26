use crate::bus::memory_map::*;
use crate::error::Error;
use crate::Bus;

/// Controls the GPIO pins on a MATRIX device.
pub struct Gpio<'a> {
    bus: &'a Bus,
}

impl<'a> Gpio<'a> {
    /// Returns an instance of GPIO.
    pub fn new(bus: &'a Bus) -> Gpio {
        Gpio { bus }
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
