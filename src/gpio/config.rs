use super::Gpio;

pub trait PinConfig {
    /// Returns a tuple with an encoded number, representing your configuration, and an FPGA address offset.
    fn generate_values(&self, pin: u8, gpio: &Gpio) -> (u32, u16);
}

/// Specifies if a pin is being used for Output or Input signals.
pub enum Mode {
    Output,
    Input,
}
impl PinConfig for Mode {
    fn generate_values(&self, pin: u8, gpio: &Gpio) -> (u32, u16) {
        let mode = match self {
            Mode::Input => 0,
            Mode::Output => 1,
        };

        let mut history = *gpio.mode_history.lock().unwrap();
        let mask = 1 << pin;
        history = mode << pin | (history & !mask);

        (history as u32, 0)
    }
}

// Specifies the current signal state of a pin.
pub enum State {
    Off,
    On,
}

impl PinConfig for State {
    fn generate_values(&self, pin: u8, gpio: &Gpio) -> (u32, u16) {
        let mode = match self {
            State::Off => 0,
            State::On => 1,
        };

        let mut history = *gpio.state_history.lock().unwrap();
        let mask = 1 << pin;
        history = mode << pin | (history & !mask);

        (history as u32, 1)
    }
}

/// Specifies which function a pin is using.
pub enum Function {
    Digital,
    Pwm,
}

impl PinConfig for Function {
    fn generate_values(&self, pin: u8, gpio: &Gpio) -> (u32, u16) {
        let mode = match self {
            Function::Digital => 0,
            Function::Pwm => 1,
        };

        let mut history = *gpio.function_history.lock().unwrap();
        let mask = 1 << pin;
        history = mode << pin | (history & !mask);

        (history as u32, 2)
    }
}
