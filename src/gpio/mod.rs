use crate::Bus;
use crate::Error;
pub mod bank;
pub mod config;
use crate::bus::memory_map::*;
pub use bank::*;
pub use config::*;
use std::mem;
use std::sync::Mutex;

/// Controls the GPIO pins on a MATRIX device.
#[derive(Debug)]
pub struct Gpio<'a> {
    bus: &'a Bus,
    /// Current setting of each pin's mode (binary representation).
    mode_pin_map: Mutex<u16>,
    /// Current setting of each pin's state (binary representation).
    state_pin_map: Mutex<u16>,
    /// Current setting of each pin's function (binary representation).
    function_pin_map: Mutex<u16>,
    /// Current setting of each bank's prescaler (binary representation).
    prescaler_bank_map: Mutex<u16>,
    /// Current state of each GPIO Bank.
    banks: Mutex<Vec<Bank<'a>>>,
}

impl<'a> Gpio<'a> {
    /// Returns an instance of GPIO.
    pub fn new(bus: &'a Bus) -> Gpio {
        Gpio {
            bus,
            mode_pin_map: Mutex::new(0x0),
            state_pin_map: Mutex::new(0x0),
            function_pin_map: Mutex::new(0x0),
            prescaler_bank_map: Mutex::new(0x0),
            banks: Mutex::new(Bank::new_set(&bus)),
        }
    }

    /// A simple check to make sure a selected pin Exists
    fn is_pin_valid(pin: u8) -> Result<(), Error> {
        if pin > 15 {
            return Err(Error::InvalidGpioPin);
        }

        Ok(())
    }
}

///////////////////////////////
// Get Functions
//////////////////////////////
impl<'a> Gpio<'a> {
    /// Returns the current digital value of a MATRIX GPIO pin (0->15).
    pub fn get_state(&self, pin: u8) -> bool {
        // TODO: add error check
        Gpio::is_pin_valid(pin).unwrap();

        // create read buffer
        let mut data: [u32; 3] = [0; 3];

        // update read buffer
        self.bus_read(&mut data, 2, 1); // all pin states are encoded as a single u16. 2 bytes needed (8*2 = 16 pins)

        // bit operation to extract the current pin's state
        let mask = 0x1 << pin;
        let state = (data[2] & mask) >> pin;

        match state {
            0 => false,
            1 => true,
            _ => {
                panic!("Error retrieving current pin state. Digital value returned was not 0 or 1")
            }
        }
    }

    // TODO: change u8 to State
    /// Returns the current digital value of every MATRIX GPIO pin (0->15)
    pub fn get_states(&self) -> [bool; 16] {
        // create read buffer
        let mut data: [u32; 3] = [0; 3];

        // update read buffer
        self.bus_read(&mut data, 2, 1); // all pin states are encoded as a single u16. 2 bytes needed (8*2 = 16 pins)

        // bit operation to extract each pin state (0-15)
        let mut pins: [bool; 16] = [false; 16];
        for i in 0..16 {
            let mask = 0x1 << i;
            let state = ((data[2] & mask) >> i) as u8;

            pins[i] = match state {
                0 => false,
                1 => true,
                _ => panic!(
                    "Error retrieving current pin state. Digital value returned was not 0 or 1"
                ),
            };
        }

        pins
    }

    /// Shortener to populate a read buffer for GPIO pin information.
    fn bus_read(&self, buffer: &mut [u32], buffer_length: u32, address_offset: u16) {
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

///////////////////////////////
// Set Functions
//////////////////////////////
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
