use super::Gpio;

pub trait EncodePinConfig {
    /// Creates a bit encoded number from a config. This is meant to be passed into a system call.
    fn encode(&self, pin: u8, gpio: &Gpio) -> u32;

    /// Address offset needed to access config in system call.
    fn fpga_address_offset(&self) -> u16;
}

/// Specifies if a pin is being used for Output or Input signals.
pub enum Mode {
    Output,
    Input,
}

impl EncodePinConfig for Mode {
    fn encode(&self, pin: u8, gpio: &Gpio) -> u32 {
        let mode = match self {
            Mode::Input => 0,
            Mode::Output => 1,
        };

        let mut history = *gpio.mode_history.lock().unwrap();
        let mask = 1 << pin;

        history = mode << pin | (history & !mask);
        history as u32
    }

    fn fpga_address_offset(&self) -> u16 {
        0
    }
}

/// Specifies which function a pin is using.
pub enum Function {
    Digital,
    Pwm,
}

impl EncodePinConfig for Function {
    fn encode(&self, pin: u8, gpio: &Gpio) -> u32 {
        let function = match self {
            Function::Digital => 0,
            Function::Pwm => 1,
        };

        let mut history = *gpio.function_history.lock().unwrap();
        let mask = 1 << pin;

        history = function << pin | (history & !mask);
        history as u32
    }

    fn fpga_address_offset(&self) -> u16 {
        2
    }
}

// Specifies the current signal state of a pin.
pub enum State {
    On,
    Off,
}

impl EncodePinConfig for State {
    fn encode(&self, pin: u8, gpio: &Gpio) -> u32 {
        let state = match self {
            State::On => 1,
            State::Off => 0,
        };

        let mut history = *gpio.value_history.lock().unwrap();
        let mask = 1 << pin;

        history = state << pin | (history & !mask);
        history as u32
    }

    fn fpga_address_offset(&self) -> u16 {
        1
    }
}
