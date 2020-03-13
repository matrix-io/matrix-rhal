pub mod kernel;
pub mod memory_map;
use crate::{Device, Error};
use std::boxed::Box;

// A type that has the ability to read/write to the MATRIX Bus on the FPGA.
pub trait MatrixBus: std::fmt::Debug {
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
    fn read(&self, write_buffer: &mut [u8]);

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
    fn write(&self, read_buffer: &mut [u8]);

    /// Close the file descriptor that's communicating with the bus' device file.
    fn close(&self);

    /// Get the number of LEDs in the everloop of the MATRIX device (the ring of LEDs).
    fn get_device_leds(&self) -> u8;

    /// Get the type of MATRIX device that's currently attached.
    fn get_device_name(&self) -> Device;

    /// Get the FPGA clock speed.
    fn get_fpga_frequency(&self) -> u32;
}

/// Return a Bus that communicates through the MATRIX Bus or an SPI interface.
pub fn init() -> Result<Box<dyn MatrixBus>, Error> {
    Ok(Box::new(kernel::Bus::init()?))
}
