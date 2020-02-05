use super::memory_map::*;
use super::sensors;
use crate::{error::Error, Device};
use nix::fcntl::{open, OFlag}; // https://linux.die.net/man/3/open
use nix::ioctl_read_bad;
use nix::sys::stat::Mode;
use nix::unistd::close; // https://linux.die.net/man/2/close
use std::sync::Mutex;

pub const WR_VALUE: i32 = 1200;
pub const RD_VALUE: i32 = 1201;

// Generate read() function
ioctl_read_bad!(read, RD_VALUE, [i32]);

// Generate write function
// TODO:

// kernel bus
pub struct Bus<'a> {
    /// Path for the device file being used. This is what's used to communicate with the MATRIX Kernel.
    pub device_file: &'a str,
    /// Read buffer passed into IOCTL read function.
    pub rx_buffer: [i32; 12288],
    pub tx_buffer: [i32; 12288],
    /// File descriptor for kernel abstraction.
    pub regmap_fd: std::os::unix::io::RawFd,
    /// Mutex to prevent collisions during read/write.
    pub usage: Mutex<()>,
    pub device_name: Device,
    pub device_leds: u8,
}

// TODO: add Error handling
impl<'a> Bus<'a> {
    /// Open the MATRIX Kernel's device file & retrieve file descriptor id.
    pub fn init(&mut self) -> Result<(), Error> {
        self.usage.lock()?;

        self.regmap_fd = open(self.device_file, OFlag::O_RDWR, Mode::empty())?;
        Ok(())
    }

    pub fn write(&self, add: u16, data: &u8, length: i32) -> bool {
        todo!();
    }

    /// Populate the Bus' rx_buffer with the requested data
    pub fn read(&mut self, add: u16, length: i32) {
        self.usage.lock();

        self.rx_buffer[0] = add as i32;
        self.rx_buffer[1] = length; // bytes

        // IOCTL read call
        unsafe {
            read(self.regmap_fd, &mut self.rx_buffer).unwrap();
        }
    }

    /// Close the file descriptor that's communicating with the MATRIX Kernel's device file.
    pub fn close(&self) {
        close(self.regmap_fd).unwrap();
    }

    /// Return the type of MATRIX device being used.
    pub fn get_device_name(&mut self) -> Result<Device, Error> {
        // store the bytes representing device type & version
        self.read(K_CONF_BASE_ADDRESS, 8);

        let device_name = self.rx_buffer[2];
        // let device_version = self.rx_buffer[3]; // currently unused

        match device_name {
            K_MATRIX_CREATOR => Ok(Device::Creator),
            K_MATRIX_VOICE => Ok(Device::Voice),
            _ => Err(Error::UnknownDevice),
        }
    }
}
