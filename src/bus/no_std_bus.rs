use super::MatrixBus;
use crate::{bus::memory_map::*, error::Error, Device};

#[derive(Debug)]
pub struct Bus {}

pub fn init() -> Result<Bus, Error> {
    unimplemented!()
}

impl MatrixBus for Bus {
    fn write(&self, write_buffer: &mut [u8]) {
        unimplemented!()
    }

    fn read(&self, read_buffer: &mut [u8]) {
        unimplemented!()
    }

    fn close(&self) {
        unimplemented!()
    }

    fn device_name(&self) -> Device {
        unimplemented!()
    }

    fn device_version(&self) -> u32 {
        unimplemented!()
    }

    fn device_leds(&self) -> u8 {
        unimplemented!()
    }

    fn fpga_frequency(&self) -> u32 {
        unimplemented!()
    }
}
