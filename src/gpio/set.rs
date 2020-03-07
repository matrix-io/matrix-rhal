use super::config::*;
use super::Gpio;
use crate::bus::memory_map::*;
use crate::error::Error;
use std::mem;

impl<'a> Gpio<'a> {
    /// Configure a specific pin's mode, function, state, etc..
    pub fn set_config<T>(&self, pin: u8, config: T) -> Result<(), Error>
    where
        T: PinConfig,
    {
        Gpio::is_pin_valid(pin)?;

        // update and send pin config to matrix bus
        let (value, fpga_address_offset) = config.update_pin_map(pin, self)?;
        self.bus_write(value, fpga_address_offset);

        Ok(())
    }

    // TODO: improve by to not have to call a mutex lock for every pin being set
    /// Configure multiple pins' mode, function, state, etc..
    pub fn set_configs<T>(&self, pins: &[u8], config: T) -> Result<(), Error>
    where
        T: PinConfig,
    {
        for pin in pins.iter() {
            // update and send pin config to matrix bus
            let (value, fpga_address_offset) = config.update_pin_map(*pin, self)?;
            self.bus_write(value, fpga_address_offset);
        }

        Ok(())
    }

    /// Shortener to set pin configurations. `value` & `address_offset` are directly passed into the bus' write buffer.
    fn bus_write(&self, value: u16, address_offset: u16) {
        // create and populate write buffer
        let mut buffer: [u32; 3] = [0; 3];
        buffer[0] = (fpga_address::GPIO + address_offset) as u32; // address to write to
        buffer[1] = mem::size_of_val(&value) as u32; // byte length of value
        buffer[2] = value as u32;

        self.bus
            .write(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(&mut buffer) });
    }

    /// Set the prescaler value for a specific bank
    pub fn set_prescaler(&self, bank: usize, prescaler: u16) -> Result<(), Error> {
        let mask = 0xF << (4 * bank);
        let mut bank_prescaler = self.prescaler_bank_map.lock()?;

        *bank_prescaler = prescaler << (4 * bank) | (*bank_prescaler & !mask);

        self.bus_write(*bank_prescaler, 3);
        Ok(())
    }

    /// Set the Pulse Width Modulation output for a pin.
    pub fn set_pwm(&self, pin: u8, frequency: f32, percentage: f32) -> Result<(), Error> {
        Gpio::is_pin_valid(pin)?;

        const GPIO_PRESCALER: u16 = 0x5;
        let period_seconds = 1.0 / frequency;
        let fpga_clock = self.bus.fpga_frequency;

        let period_counter: u32 =
            ((period_seconds * fpga_clock as f32) / ((1 << GPIO_PRESCALER) * 2) as f32) as u32;

        let duty_counter = ((period_counter as f32 * percentage) / 100.0) as u16;
        let bank = (pin / 4) as u16;
        let channel = (pin % 4) as u16;

        // apply PWM settings
        self.set_prescaler(bank as usize, GPIO_PRESCALER)?;
        let bank = &self.banks.lock()?[0];
        bank.set_period(period_counter as u16);
        bank.set_duty(channel, duty_counter);

        Ok(())
    }
}
