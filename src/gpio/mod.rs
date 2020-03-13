use crate::Bus;
use crate::Error;
pub mod bank;
pub mod config;
use crate::bus::memory_map::*;
pub use bank::*;
pub use config::*;
use core::intrinsics::transmute;
use core::sync::atomic::{AtomicU16, Ordering};

/// Controls the GPIO pins on a MATRIX device.
#[derive(Debug)]
pub struct Gpio<'a> {
    bus: &'a Bus,
    /// Current setting of each pin's mode (binary representation).
    mode_pin_map: AtomicU16,
    /// Current setting of each pin's state (binary representation).
    state_pin_map: AtomicU16,
    /// Current setting of each pin's function (binary representation).
    function_pin_map: AtomicU16,
    /// Current setting of each bank's prescaler (binary representation).
    prescaler_bank_map: AtomicU16,
    /// Current state of each GPIO Bank.
    banks: [Bank<'a>; 4],
}

impl<'a> Gpio<'a> {
    /// Returns an instance of GPIO.
    pub fn new(bus: &'a Bus) -> Gpio {
        Gpio {
            bus,
            mode_pin_map: AtomicU16::new(0x0),
            state_pin_map: AtomicU16::new(0x0),
            function_pin_map: AtomicU16::new(0x0),
            prescaler_bank_map: AtomicU16::new(0x0),
            banks: Bank::new_set(&bus),
        }
    }

    /// A quick check to make sure a selected pin exists. Pins available are from `0-15`.
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

    /// Shortener to populate a read buffer, through `bus.read`, for GPIO pin information.
    fn bus_read(&self, buffer: &mut [u32], buffer_length: u32, address_offset: u16) {
        // address to query
        buffer[0] = (fpga_address::GPIO + address_offset) as u32;
        // size of expected data (bytes)
        buffer[1] = buffer_length;

        // populate buffer
        // the buffer will be passed a value that contains the state of each GPIO pin
        self.bus
            .read(unsafe { transmute::<&mut [u32], &mut [u8]>(buffer) });
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

    // TODO: improve not having to call a mutex lock for every pin being set
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

    /// Shortener to send pin configurations through `bus.write`.
    fn bus_write(&self, value: u16, address_offset: u16) {
        let mut buffer: [u32; 3] = [0; 3];
        buffer[0] = (fpga_address::GPIO + address_offset) as u32; // address to write to
        buffer[1] = 2; // byte length of u16 value
        buffer[2] = value as u32;

        self.bus
            .write(unsafe { transmute::<&mut [u32], &mut [u8]>(&mut buffer) });
    }

    /// Set the prescaler value for a specific bank
    pub fn set_prescaler(&self, bank: usize, prescaler: u16) -> Result<(), Error> {
        let mask = 0xF << (4 * bank);
        let new_prescaler = loop {
            let bank_prescaler = self.prescaler_bank_map.load(Ordering::Acquire);

            let new_prescaler = prescaler << (4 * bank) | (bank_prescaler & !mask);
            if self.prescaler_bank_map.compare_and_swap(
                bank_prescaler,
                new_prescaler,
                Ordering::Release,
            ) == bank_prescaler
            {
                break new_prescaler;
            }
        };

        self.bus_write(new_prescaler, 3);
        Ok(())
    }

    /// Set the Pulse Width Modulation output for a pin.
    pub fn set_pwm(&self, pin: u8, frequency: f32, percentage: f32) -> Result<(), Error> {
        Gpio::is_pin_valid(pin)?;

        const GPIO_PRESCALER: u16 = 0x5;
        let period_seconds = 1.0 / frequency;
        let fpga_clock = self.bus.fpga_frequency();

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

    /// Abstraction over `set_pwm` to easily control a servo.
    ///
    /// `min_pulse_ms` accepts values from `0` to `1.5`. Inputs outside this range will be set to the closest valid number.
    pub fn set_servo_angle(&self, pin: u8, angle: u32, min_pulse_ms: f32) -> Result<(), Error> {
        Gpio::is_pin_valid(pin)?;

        // prevent min_pulse_ms from exceeding the valid range
        let mut min_pulse_ms = min_pulse_ms;
        if min_pulse_ms > 1.5 {
            min_pulse_ms = 1.5;
        } else if min_pulse_ms < 0.0 {
            min_pulse_ms = 0.0;
        }

        // We choose a prescaler of 32 to work with a lower frequency
        const GPIO_PRESCALER: u16 = 0x5;

        // We need 50Hz for servo, so 1 / 50Hz = 0.02 sec (https://en.wikipedia.org/wiki/Servo_(radio_control))
        const PERIOD_SECONDS: f32 = 0.02;

        /*
        Getting period_counter to generate 50Hz:
        FPGAClock = 150000000
        FPGAClockAfterPrescaler = 150000000 / 32 = 4687500
        Period counter required for 50Hz
        period_counter = 0.02 / ( 1 / 4687500 ) = 93750
        FPGA firmware need only half of the period counter
        half_period_counter = period_counter / 2 = 46875
        When all math is combined you get
        final_period_counter = (period_seconds * FPGAClock / ((1 << GPIOPrescaler) * 2);
        */
        let period_counter: u32 = ((PERIOD_SECONDS * self.bus.fpga_frequency() as f32)
            / (((1 << GPIO_PRESCALER) * 2) as f32)) as u32;

        // Servo pulse width is symmetrical, with 1.5ms as neutral position
        // 1.5ms / 20ms = 0.075
        let servo_middle = (period_counter as f32 * 0.075) as u32;
        let servo_offset = (period_counter as f32 * (min_pulse_ms / 20.0)) as u32;
        let servo_ratio = (servo_middle - servo_offset) / 90;

        let duty_counter = (servo_ratio * angle as u32) + servo_offset;

        let bank = pin / 4;
        let channel = pin % 4;

        // apply PWM for desired servo angle
        self.set_prescaler(bank as usize, GPIO_PRESCALER)?;
        let bank = &self.banks.lock()?[0];
        bank.set_period(period_counter as u16);
        bank.set_duty(channel as u16, duty_counter as u16);

        Ok(())
    }
}
