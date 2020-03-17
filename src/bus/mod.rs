pub mod memory_map;
use crate::{with_std, without_std, Device};

with_std! {
    mod std_bus;
    pub use std_bus::init;
}

without_std! {
    mod no_std_bus;
    pub use no_std_bus::init;
}

pub trait MatrixBus {
    /// Send a write buffer to the MATRIX Bus. The buffer requires an `address` to request,
    /// the `byte_length` of the data being given, and then the rest of the data itself.
    ///
    /// # Usage
    ///  ```
    ///  let bus = matrix_rhal::Bus::init().unwrap();
    ///
    ///  # let address_offset = 0;
    ///  let some_value: u16 = 237;
    ///  let mut buffer: [u32; 3] = [0; 3];
    ///     
    ///  // address to query
    ///  buffer[0] = (matrix_rhal::bus::memory_map::fpga_address::GPIO + address_offset) as u32;
    ///  // byte length of data (u16 = 2 bytes)
    ///  buffer[1] = 2;
    ///  // data being sent
    ///  buffer[2] = some_value as u32;
    ///
    ///  // send buffer
    ///  bus.write(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(&mut buffer) });
    ///  ```
    fn write(&self, write_buffer: &mut [u8]);

    /// Send a read buffer to the MATRIX Bus. The buffer requires an `address` to request and
    /// the `byte_length` of what's expected to be returned. Once sent, the buffer return with populated
    /// data.
    ///
    /// Keep in mind, the returned buffer will still have the `address` and `byte_length` that was passed.
    ///
    /// # Usage
    ///  ```
    ///  let bus = matrix_rhal::Bus::init().unwrap();
    ///  let mut buffer: [u32; 4] = [0; 4];
    ///
    ///  // address to query
    ///  buffer[0] = (matrix_rhal::bus::memory_map::fpga_address::CONF) as u32;
    ///  // bytes being requested
    ///  buffer[1] = 8;
    ///
    ///  // populate buffer
    ///  bus.read(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(&mut buffer) });
    ///
    ///  // returned data will start at buffer[2]
    ///  println!("{:?}", buffer);
    ///  ```
    fn read(&self, read_buffer: &mut [u8]);

    /// If possible, close the connection to the MATRIX Bus.
    fn close(&self);

    fn device_name(&self) -> Device;
    fn device_version(&self) -> u32;
    fn device_leds(&self) -> u8;
    fn fpga_frequency(&self) -> u32;
}
