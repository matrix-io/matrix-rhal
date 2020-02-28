/// Specifies if a pin is being used for Output or Input signals.
pub enum Mode {
    Output,
    Input,
}

impl EncodePinConfig for Mode {
    fn encode(&self, pin: u8) -> u32 {
        let mode = match self {
            Mode::Input => 0,
            Mode::Output => 1,
        };

        let mask = 1 << pin;
        mode << pin | (0x0 & !mask)
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
    fn encode(&self, pin: u8) -> u32 {
        let function = match self {
            Function::Digital => 0,
            Function::Pwm => 1,
        };

        let mask = 1 << pin;
        function << pin | (0x0 & !mask)
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
    fn encode(&self, pin: u8) -> u32 {
        let state = match self {
            State::On => 1,
            State::Off => 0,
        };

        let mask = 1 << pin;
        state << pin | (0x0 & !mask)
    }

    fn fpga_address_offset(&self) -> u16 {
        1
    }
}

pub trait EncodePinConfig {
    /// Creates a bit encoded number from a config. This is meant to be passed into a system call.
    fn encode(&self, pin: u8) -> u32;

    /// Address offset needed to access config in system call.
    fn fpga_address_offset(&self) -> u16;
}
