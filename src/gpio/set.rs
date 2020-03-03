use super::config::*;
use super::Gpio;
use crate::bus::memory_map::*;

impl<'a> Gpio<'a> {
    /// Configure a specific pin's mode, function, state, etc..
    pub fn set_config<T>(&self, pin: u8, config: T)
    where
        T: PinConfig,
    {
        if pin > 15 {
            panic!("The MATRIX Voice/Creator GPIO pins are from 0-15");
        }

        // send pin config to matrix bus
        let (value, fpga_address_offset) = config.generate_values(pin, self);
        self.pin_set(value, fpga_address_offset);
    }

    /// Configure multiple pins' mode, function, state, etc..
    pub fn set_configs<T>(&self, pins: &[u8], config: T)
    where
        T: PinConfig,
    {
        for pin in pins.iter() {
            // send pin config to matrix bus
            let (value, fpga_address_offset) = config.generate_values(*pin, self);
            self.pin_set(value, fpga_address_offset);
        }
    }

    /// Shortener to set pin configurations. `value` & `address_offset` are directly passed into the bus' write buffer.
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
