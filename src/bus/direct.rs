use super::{memory_map::*, MatrixBus};
use crate::{error::Error, info, Device};
use nix::{request_code_read, request_code_write}; // IOR and IOW,

use nix::fcntl::{open, OFlag}; // https://linux.die.net/man/3/open
use nix::sys::stat::Mode;
use nix::unistd::close;
use nix::{ioctl_read_bad, ioctl_write_ptr_bad};

/// SPI Magic number for IOCTL
const SPI_IOC_MAGIC: u8 = b'k'; // Defined in linux/spi/spidev.h

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

        let spi_ioc_rd_mode = request_code_read!(SPI_IOC_MAGIC, 1, 1);
        assert_eq!(spi_ioc_rd_mode, 2147576577);

        let spi_ioc_rd_bits_per_word = request_code_read!(SPI_IOC_MAGIC, 3, 1);
        assert_eq!(spi_ioc_rd_bits_per_word, 2147576579);

        let spi_ioc_rd_max_speed_hz = request_code_read!(SPI_IOC_MAGIC, 4, 4);
        assert_eq!(spi_ioc_rd_max_speed_hz, 2147773188);

        // * Write Numbers
        let spi_ioc_wr_mode = request_code_write!(SPI_IOC_MAGIC, 1, 1);
        assert_eq!(spi_ioc_wr_mode, 1073834753);

        let spi_ioc_wr_bits_per_word = request_code_write!(SPI_IOC_MAGIC, 3, 1);
        assert_eq!(spi_ioc_wr_bits_per_word, 1073834755);

        let spi_ioc_wr_max_speed_hz = request_code_write!(SPI_IOC_MAGIC, 4, 4);
        assert_eq!(spi_ioc_wr_max_speed_hz, 1074031364);

        // if (ioctl(spi_fd_, SPI_IOC_WR_MODE, &spi_mode_) == -1) {
        //     std::cerr << "can't set spi mode" << std::endl;
        //     return false;
        // }

        println!("This passed all according to plan!");
        Err(Error::InvalidGpioPin)
    }
}

impl MatrixBus for Bus {
    fn write(&self, write_buffer: &mut [u8]) {
        unsafe {
            // TODO: ....
        }
    }

    fn read(&self, read_buffer: &mut [u8]) {
        unsafe {
            // TODO: ....
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
