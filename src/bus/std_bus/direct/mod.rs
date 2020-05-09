use super::super::{memory_map::*, MatrixBus};
pub mod ioctl;
use crate::{error::Error, info, Device};
use ioctl::*;
use nix::fcntl::{open, OFlag}; // https://linux.die.net/man/3/open
use nix::sys::stat::Mode;
use nix::unistd::close;

/// Bridge for talking to the FPGA on a MATRIX device.
/// Most, if not all, MATRIX functionality requires this Bus to read and write data.
#[derive(Debug)]
pub struct Bus {
    /// Path for the device file being used. This is what's used to communicate with the Raspberry Pi's SPI.
    pub device_file: &'static str,
    /// File descriptor for the SPI device file.
    pub regmap_fd: std::os::unix::io::RawFd,
    /// Type of MATRIX device that's currently attached.
    pub device_name: Device,
    /// The version of the board.
    pub device_version: u32,
    /// Number of LEDS on the MATRIX device.
    pub device_leds: u8,
    /// Frequency of the FPGA on the MATRIX device.
    pub fpga_frequency: u32,

    pub spi_mode: u32,
    pub spi_bits: u32,
    pub spi_speed: u32,
    pub spi_delay: u32,
}

impl Bus {
    pub fn init() -> Result<Bus, Error> {
        let mut bus = Bus {
            device_file: "/dev/spidev0.0",
            regmap_fd: 0,
            device_name: Device::Unknown,
            device_version: 0,
            device_leds: 0,
            fpga_frequency: 0,

            spi_mode: 3,
            spi_bits: 8,
            spi_speed: 150_000_00,
            spi_delay: 0,
        };

        // open the file descriptor to communicate with the MATRIX kernel
        bus.regmap_fd = open(bus.device_file, OFlag::O_RDWR, Mode::empty())?;

        // configure SPI settings
        unsafe {
            spi_ioc_write_mode(bus.regmap_fd, &mut bus.spi_mode).unwrap();
            spi_ioc_read_mode(bus.regmap_fd, &mut bus.spi_mode).unwrap();

            spi_ioc_write_bits_per_word(bus.regmap_fd, &mut bus.spi_bits).unwrap();
            spi_ioc_read_bits_per_word(bus.regmap_fd, &mut bus.spi_bits).unwrap();

            spi_ioc_write_max_speed_hz(bus.regmap_fd, &mut bus.spi_speed).unwrap();
            spi_ioc_read_max_speed_hz(bus.regmap_fd, &mut bus.spi_speed).unwrap();
        }

        println!("This passed all according to plan!");
        Ok(bus)
    }

    // fn spi_transfer(&self, send_buffer: &mut [u8], receive_buffer: &mut [u8], bytes: u32) {}
}

impl MatrixBus for Bus {
    fn write(&self, address: u16, write_buffer: &[u8]) {
        unsafe {
            // TODO: ....
            unimplemented!()
        }
    }

    fn read(&self, address: u16, read_buffer: &mut [u8]) {
        unsafe {
            // TODO: ....
            unimplemented!()
        }
    }

    fn close(&self) {
        close(self.regmap_fd).unwrap();
    }

    fn device_name(&self) -> Device {
        self.device_name
    }

    fn device_version(&self) -> u32 {
        self.device_version
    }

    fn device_leds(&self) -> u8 {
        self.device_leds
    }

    fn fpga_frequency(&self) -> u32 {
        self.fpga_frequency
    }
}
