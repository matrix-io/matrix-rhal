use crate::bus::memory_map::*;
use crate::Bus;
use core::intrinsics::transmute;
use heapless::{consts, Vec};

/// Bank contains functions to configure a PWM.
/// A bank is a set of 4 pins, starting from pin 0 and going in order.
///
/// Bank 0: pins (0->3)
///
/// Bank 1: pins (4->8)
///
/// Bank 2: pins (9->12)
///
/// Bank 3: pins (13->16)
#[derive(Debug, Clone)]
pub struct Bank<'a> {
    bus: &'a Bus,
    /// FPGA memory offset
    pub memory_offset: u16,
    pub timer_setup: u16,
}

impl<'a> Bank<'a> {
    /// Create a new instance of GPIO Bank.
    pub fn new(bus: &Bus) -> Bank {
        Bank {
            bus,
            memory_offset: 0x0,
            timer_setup: 0x0,
        }
    }

    /// Create 4 banks configured for use in a MATRIX device.
    pub fn new_set(bus: &Bus) -> [Bank; 4] {
        let mut gpio_base_address = fpga_address::GPIO + 4;

        // create a bank for each set of 4 pins
        let mut banks = [{
            // configure each bank with the proper address offsets
            let mut bank = Bank::new(&bus);
            bank.memory_offset = gpio_base_address;
            gpio_base_address += 6;
            bank
        }; 4];

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
        let mut buffer: [u32; 3] = [0; 3];
        buffer[0] = (memory_offset) as u32; // address to write to
        buffer[1] = 2; // byte length of timer_setup
        buffer[2] = timer_setup as u32;

        self.bus
            .write(unsafe { transmute::<&mut [u32], &mut [u8]>(&mut buffer) });
    }
}
