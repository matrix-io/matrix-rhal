use crate::bus::memory_map;
use crate::Bus;

pub struct Gpio<'a> {
    bus: &'a Bus,
}

impl<'a> Gpio<'a> {
    pub fn new(bus: &'a Bus) -> Gpio {
        Gpio { bus }
    }

    pub fn get_value(self, pin: u8) -> u8 {
        let mask = 0x1 << pin;

        // uint16_t value;
        // bus_->Read(kGPIOBaseAddress + 1, &value);
        // value = (value & mask) >> pin;

        // return value;

        0
    }
}
