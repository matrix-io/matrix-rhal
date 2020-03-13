use super::memory_map;
use super::MatrixBus;
use crate::{error::Error, info, Device};
use memory_map::*;
use nix::fcntl::{open, OFlag}; // https://linux.die.net/man/3/open
use nix::sys::stat::Mode;
use nix::unistd::close;
use nix::{ioctl_read_bad, ioctl_write_ptr_bad};

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
    /// The version of the board.
    pub device_version: u32,
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
            device_version: 0,
            device_leds: 0,
            fpga_frequency: 0,
        };

        // open the file descriptor to communicate with the MATRIX kernel
        bus.regmap_fd = open(bus.device_file, OFlag::O_RDWR, Mode::empty())?;

        // fetch information on the current MATRIX device
        let (name, version) = info::get_device_info(&bus)?;
        bus.device_name = name;
        bus.device_version = version;

        bus.device_leds = match bus.device_name {
            Device::Creator => device_info::MATRIX_CREATOR_LEDS,
            Device::Voice => device_info::MATRIX_VOICE_LEDS,
            _ => panic!("Cannot determine number of LEDs on device (This is a hard-coded value)."),
        };
        bus.fpga_frequency = info::get_fpga_frequency(&bus)?;

        Ok(bus)
    }
}

impl MatrixBus for Bus {
    fn write(&self, write_buffer: &mut [u8]) {
        unsafe {
            // TODO: error handling. Not sure if an error here would be worth recovering from.
            ioctl_write(self.regmap_fd, write_buffer).expect("error in IOCTL WRITE");
        }
    }

    fn read(&self, read_buffer: &mut [u8]) {
        unsafe {
            // TODO: error handling. Not sure if an error here would be worth recovering from.
            ioctl_read(self.regmap_fd, read_buffer).expect("error in IOCTL READ");
        }
    }

    fn close(&self) {
        close(self.regmap_fd).unwrap();
    }

    fn get_device_leds(&self) -> u8 {
        self.device_leds
    }

    fn get_device_name(&self) -> Device {
        self.device_name
    }

    fn get_fpga_frequency(&self) -> u32 {
        self.fpga_frequency
    }
}
