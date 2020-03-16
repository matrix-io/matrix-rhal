pub mod memory_map;
use crate::{with_std, without_std, Device};

with_std! {
    mod std_bus;
    pub use std_bus::Bus as Bus;
}

without_std! {
    mod no_std_bus;
    pub use no_std_bus::Bus as Bus;
}

trait MatrixBus {
    fn write(&self, write_buffer: &mut [u8]);
    fn read(&self, read_buffer: &mut [u8]);
    fn close(&self);

    fn device_name(&self) -> Device;
    fn device_version(&self) -> u32;
    fn device_leds(&self) -> u8;
    fn fpga_frequency(&self) -> u32;
}
