use super::Gpio;
use crate::bus::memory_map::*;

impl<'a> Gpio<'a> {
    /// Returns the digital value of a MATRIX GPIO pin (0-15).
    pub fn get_value(&self, pin: u8) -> u8 {
        // create read buffer
        let mut data: [u32; 3] = [0; 3];

        // update read buffer
        self.pin_get(&mut data, 2, 1); // all pin states are encoded as a single u16. 2 bytes needed (8*2 = 16 pins)

        // bit operation to extract the current pin's state
        let mask = 0x1 << pin;
        ((data[2] & mask) >> pin) as u8
    }

    /// Returns the current state of every MATRIX GPIO pin (0-15)
    pub fn get_values(&self) -> [u8; 16] {
        // create read buffer
        let mut data: [u32; 3] = [0; 3];

        // update read buffer
        self.pin_get(&mut data, 2, 1); // all pin states are encoded as a single u16. 2 bytes needed (8*2 = 16 pins)

        // bit operation to extract each pin state (0-15)
        let mut pins = [0; 16];
        for i in 0..16 {
            let mask = 0x1 << i;
            pins[i] = ((data[2] & mask) >> i) as u8
        }

        pins
    }

    /// Shortener to populate a read buffer for GPIO pin information.
    fn pin_get(&self, buffer: &mut [u32], buffer_length: u32, address_offset: u16) {
        // address to query
        buffer[0] = (fpga_address::GPIO + address_offset) as u32;
        // size of expected data (bytes)
        buffer[1] = buffer_length;

        // populate buffer
        // the buffer will be passed a value that contains the state of each GPIO pin
        self.bus
            .read(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(buffer) });
    }
}
