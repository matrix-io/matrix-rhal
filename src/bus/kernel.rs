use super::memory_map::*;
use crate::Device;
use nix::fcntl::{open, OFlag}; // https://linux.die.net/man/3/open
use nix::sys::stat::Mode;
use nix::unistd::close; // https://linux.die.net/man/2/close
use nix::*;
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
    pub rx_buffer: [i32; 12288],
    pub tx_buffer: [i32; 12288],
    /// File descriptor for kernel abstraction.
    pub regmap_fd: std::os::unix::io::RawFd,
    // Empty because we don't need to pass any data (yet).
    pub usage: Mutex<()>,
}

// TODO: add Error handling
impl<'a> Bus<'a> {
    /// Open the MATRIX Kernel's device file & retrieve file descriptor id.
    pub fn init(&mut self) -> bool {
        self.usage.lock().unwrap();
        self.regmap_fd = open(self.device_file, OFlag::O_RDWR, Mode::empty()).unwrap();
        true
    }

    pub fn write(&self, add: u16, data: &u8, length: i32) -> bool {
        todo!();
    }

    pub fn read(&mut self, add: u16, length: i32) {
        self.usage.lock();

        self.rx_buffer[0] = add as i32;
        self.rx_buffer[1] = length; // bytes

        unsafe {
            read(self.regmap_fd, &mut self.rx_buffer).unwrap(); // todo: replicate the memcopy: https://github.com/matrix-io/matrix-creator-hal/blob/125b75a4256db56533a9227b5c80cbe2c96b11ab/cpp/driver/bus_kernel.cpp#L72
        }
    }

    /// Close the file descriptor that's communicating with the MATRIX Kernel's device file.
    pub fn close(&self) {
        close(self.regmap_fd).unwrap();
    }

    /// Return the
    pub fn get_device_name(&mut self) -> Device {
        // store the bytes representing device type & version
        self.read(K_CONF_BASE_ADDRESS, 8);

        let device_name = self.rx_buffer[2];
        // let device_version = self.rx_buffer[3];

        match device_name {
            K_MATRIX_CREATOR => Device::Creator,
            K_MATRIX_VOICE => Device::Voice,
            _ => panic!("COULD NOT FIND DEVICE"),
        }
    }
}
