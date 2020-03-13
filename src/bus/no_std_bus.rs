use crate::{bus::memory_map::*, error::Error, Device};
use super::BusImpl;

#[derive(Debug)]
pub struct Bus {
}

impl Bus {
    pub fn init() -> Result<Bus, Error> {
        unimplemented!()
    }
    pub fn device_name(&self) -> Device {
        unimplemented!()
    }
    pub fn device_version(&self) -> u32 {
        unimplemented!()
    }
    pub fn device_leds(&self) -> u8 {
        unimplemented!()
    }
    pub fn fpga_frequency(&self) -> u32 {
        unimplemented!()
    }
    pub fn write(&self, write_buffer: &mut [u8]) {
        unimplemented!()
    }
    pub fn read(&self, read_buffer: &mut [u8]) {
        unimplemented!()
    }
    pub fn close(&self) {
        unimplemented!()
    }
}
