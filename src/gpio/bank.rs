use crate::as_u8_slice;
use crate::bus::{memory_map::*, MatrixBus};

/// Bank contains functions to configure a PWM.
/// A bank is a set of 4 pins, starting from pin 0 and going in order.
///
/// Bank 0: pins (0->3)
///
/// Bank 1: pins (4->7)
///
/// Bank 2: pins (8->11)
///
/// Bank 3: pins (12->15)
#[derive(Clone, Copy)]
pub struct Bank<'a> {
    bus: &'a dyn MatrixBus,
    /// FPGA memory offset
    pub memory_offset: u16,
    pub timer_setup: u16,
}

impl<'a> Bank<'a> {
    /// Create a new instance of GPIO Bank.
    pub fn new(bus: &dyn MatrixBus) -> Bank {
        Bank {
            bus,
            memory_offset: 0x0,
            timer_setup: 0x0,
        }
    }

    /// Create 4 banks configured for use in a MATRIX device.
    pub fn new_set(bus: &dyn MatrixBus) -> [Bank; 4] {
        let mut gpio_base_address = fpga_address::GPIO + 4;

        // create a bank for each set of 4 pins
        let mut banks = [Bank::new(bus); 4];

        // configure each bank with the proper address offsets
        for i in 0..4 {
            banks[i].memory_offset = gpio_base_address;
            gpio_base_address += 6;
        }

        banks
    }

    /// Set the period for PWM.
    pub fn set_period(&self, period: u16) {
        self.bus_write(self.memory_offset + 1, period);
    }

    /// Set the duty cycle for PWM.
    pub fn set_duty(&self, channel: u16, duty: u16) {
        self.bus_write(self.memory_offset + 2 + channel, duty);
    }

    /// Send a bank configuration to the MATRIX bus.
    fn bus_write(&self, memory_offset: u16, timer_setup: u16) {
        // create and populate write buffer
        let buffer = [timer_setup; 1];

        self.bus.write(memory_offset, as_u8_slice(&buffer));
    }
}
