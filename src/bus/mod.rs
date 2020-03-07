pub mod memory_map;
use crate::{error::Error, Device};
use memory_map::*;
use nix::fcntl::{open, OFlag}; // https://linux.die.net/man/3/open
use nix::sys::stat::Mode;
use nix::unistd::close;
use nix::{ioctl_read_bad, ioctl_write_ptr_bad};
use std::sync::Mutex;

// Generate ioctl_read() function
ioctl_read_bad!(ioctl_read, ioctl_code::READ, [u8]);

// Generate ioctl_write() function
ioctl_write_ptr_bad!(ioctl_write, ioctl_code::WRITE, [u8]);

/// Bridge for talking to the MATRIX Kernel Modules.
/// Most, if not all, MATRIX functionality requires this Bus to read and write data.
#[derive(Debug)]
pub struct Bus {
    /// Path for the device file being used. This is what's used to communicate with the MATRIX Kernel.
    pub device_file: &'static str,
    /// File descriptor for kernel abstraction.
    pub regmap_fd: std::os::unix::io::RawFd,
    /// Type of MATRIX device that's currently attached.
    pub device_name: Device,
    /// Number of LEDS on the MATRIX device.
    pub device_leds: u8,
    /// Frequency of the FPGA on the MATRIX device.
    pub fpga_frequency: u32,
}

impl Bus {
    /// Create, initialize, and return a MATRIX Bus
    pub fn init() -> Result<Bus, Error> {
        let mut bus = Bus {
            device_file: "/dev/matrixio_regmap",
            regmap_fd: 0,
            device_name: Device::Unknown,
            device_leds: 0,
            fpga_frequency: 0,
        };

        // open the file descriptor to communicate with the MATRIX kernel
        bus.regmap_fd = open(bus.device_file, OFlag::O_RDWR, Mode::empty())?;

        // fetch information on the current MATRIX device
        bus.device_name = bus.get_device_name()?;
        bus.device_leds = match bus.device_name {
            Device::Creator => 35,
            Device::Voice => 18,
            _ => panic!("Cannot determine number of LEDs on device (This is a hard-coded value)."),
        };
        bus.fpga_frequency = bus.get_fpga_frequency()?;

        Ok(bus)
    }

    /// Populate a buffer with the requested data.
    ///
    ///  `write_buffer` requires [FPGA address, bytes of data being sent, data being sent...]
    ///
    /// Any data added should start at index 2.
    pub fn write(&self, write_buffer: &mut [u8]) {
        unsafe {
            // TODO: error handling
            ioctl_write(self.regmap_fd, write_buffer).expect("error in IOCTL WRITE");
        }
    }

    /// Populate a buffer with the requested data.
    ///
    ///  `read_buffer` requires [FPGA address, bytes of data returning, all data returned...]
    ///
    /// Any data returned will start at the index 2.
    pub fn read(&self, read_buffer: &mut [u8]) {
        unsafe {
            // TODO: error handling
            ioctl_read(self.regmap_fd, read_buffer).expect("error in IOCTL READ");
        }
    }

    /// Close the file descriptor that's communicating with the MATRIX Kernel's device file.
    pub fn close(&self) {
        close(self.regmap_fd).unwrap();
    }

    /// Return the type of MATRIX device being used.
    fn get_device_name(&self) -> Result<Device, Error> {
        let mut data: [i32; 4] = [0; 4];
        data[0] = fpga_address::CONF as i32; // address to request
        data[1] = 8; // bytes needed for results (device_name(4 bytes) and device_version(4 bytes))

        // update data with values found in address
        self.read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });

        let device_name = data[2];
        // let device_version = self.rx_buffer[3]; // currently unused

        match device_name {
            device_info::MATRIX_CREATOR => Ok(Device::Creator),
            device_info::MATRIX_VOICE => Ok(Device::Voice),
            _ => Err(Error::UnknownDevice),
        }
    }

    /// Updates the Bus to have the last known FPGA frequency of the MATRIX device.
    fn get_fpga_frequency(&self) -> Result<u32, Error> {
        let mut data: [i32; 3] = [0; 3];
        data[0] = (fpga_address::CONF + 4) as i32; // address to request
        data[1] = 4; // bytes expected to be added to data

        self.read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });

        let value0 = data[2] >> 16; // store 1st 16 bits
        let value1 = !(value0 << 16) & data[2]; // store 2nd 16 bits
        let frequency = (device_info::FPGA_CLOCK * value0 as u32) / value1 as u32;

        Ok(frequency)
    }
}
