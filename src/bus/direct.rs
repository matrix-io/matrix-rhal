/// SPI Magic number for IOCTL
const SPI_IOC_MAGIC: char = b'k'; // Defined in linux/spi/spidev.h

/// Bridge for talking to the FPGA on a MATRIX device.
/// Most, if not all, MATRIX functionality requires this Bus to read and write data.
struct Bus {
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

    spi_fd: i32,
    spi_mode: u32,
    spi_bits: u32,
    spi_speed: u32,
    spi_delay: u32,
}
