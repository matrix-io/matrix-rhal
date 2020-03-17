use super::super::super::memory_map::*;
use nix::{ioctl_read, ioctl_read_bad, ioctl_write_int, ioctl_write_ptr_bad};
use nix::{request_code_read, request_code_write}; // IOR and IOW,

// Generate ioctl_read() functions
ioctl_read_bad!(ioctl_read, ioctl_code::READ, [u8]);

// Generate ioctl_write() function
ioctl_write_ptr_bad!(ioctl_write, ioctl_code::WRITE, [u8]);

/// SPI request code for IOCTL
const SPI_IOC_MAGIC: u8 = b'k'; // Defined in linux/spi/spidev.h

/*
  Below are multiple IOCTL macros to configure the Raspberry Pi's SPI to work with the MATRIX device attached
*/
// - Mode - \\
ioctl_read_bad!(
    spi_ioc_read_mode,
    request_code_read!(SPI_IOC_MAGIC, 1, 1),
    u32
);
ioctl_write_ptr_bad!(
    spi_ioc_write_mode,
    request_code_write!(SPI_IOC_MAGIC, 1, 1),
    u32
);

// - Bits Per Word - \\
ioctl_read_bad!(
    spi_ioc_read_bits_per_word,
    request_code_read!(SPI_IOC_MAGIC, 3, 1),
    u32
);

ioctl_write_ptr_bad!(
    spi_ioc_write_bits_per_word,
    request_code_read!(SPI_IOC_MAGIC, 3, 1),
    u32
);

// - Max Speed Hz - \\
ioctl_read_bad!(
    spi_ioc_read_max_speed_hz,
    request_code_read!(SPI_IOC_MAGIC, 4, 4),
    u32
);

ioctl_write_ptr_bad!(
    spi_ioc_write_max_speed_hz,
    request_code_read!(SPI_IOC_MAGIC, 4, 4),
    u32
);
