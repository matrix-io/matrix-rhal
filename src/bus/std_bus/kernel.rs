use super::super::MatrixBus;
use crate::{as_mut_u8_slice, as_u8_slice, bus::memory_map::*, error::Error, Device, info};
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
    device_file: &'static str,
    /// File descriptor for kernel abstraction.
    regmap_fd: std::os::unix::io::RawFd,
    /// Type of MATRIX device that's currently attached.
    device_name: Device,
    /// The version of the board.
    device_version: u32,
    /// Number of LEDS on the MATRIX device.
    device_leds: u8,
    /// Frequency of the FPGA on the MATRIX device.
    fpga_frequency: u32,
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

// Calculate number of i32 needed to contain:
// - Address
// - Buffer size
// - buffer.len() bytes
fn ioctl_buffer_i32(buffer: &[u8]) -> usize {
    const SIZE_OF_I32: usize = core::mem::size_of::<i32>();
    // `(x + y - 1) / y` is the same as `ceiling(x as f32/y as f32) as usize`:
    // |buffer.len() | return
    // |-|-|
    // | 0     | 2 + 0
    // | 1 - 4 | 2 + 1
    // | 5 - 8 | 2 + 2
    2 + (buffer.len() + SIZE_OF_I32 - 1) / SIZE_OF_I32
}

impl MatrixBus for Bus {
    fn write(&self, address: u16, write_buffer: &[u8]) {
        unsafe {
            // Pack request into word-sized/aligned buffer
            // Bytes:
            // [0..4] = address
            // [4..8] = byte size of write payload
            // [8..8+sizeof(payload)] = payload
            let write_buffer = {
                let mut retval = vec![0i32; ioctl_buffer_i32(write_buffer)];
                retval[0] = address as i32;
                retval[1] = write_buffer.len() as i32;
                as_mut_u8_slice(&mut retval[2..])[..write_buffer.len()].copy_from_slice(write_buffer);
                retval
            };
            // TODO: error handling. Not sure if an error here would be worth recovering from.
            ioctl_write(self.regmap_fd, as_u8_slice(&write_buffer[..])).expect("error in IOCTL WRITE");
        }
    }

    fn read(&self, address: u16, read_buffer: &mut [u8]) {
        unsafe {
            // Pack request into word-sized/aligned buffer
            // Bytes:
            // [0..4] = address
            // [4..8] = byte size of read request
            // [8..8+sizeof(payload)] = destination for read payload
            let mut buffer = {
                let mut retval = vec![0i32; ioctl_buffer_i32(read_buffer)];
                retval[0] = address as i32;
                retval[1] = read_buffer.len() as i32;
                retval
            };
            // TODO: error handling. Not sure if an error here would be worth recovering from.
            ioctl_read(self.regmap_fd, as_mut_u8_slice(&mut buffer[..])).expect("error in IOCTL READ");
            // Copy read data back into original argument buffer
            read_buffer.copy_from_slice(
                &as_u8_slice(
                    // Skip address and size words
                    &buffer[2..]
                )
                // Limit to size of destination buffer (request was word-aligned)
                [..read_buffer.len()]
            );
        }
    }

    /// Close the file descriptor that's communicating with the MATRIX Kernel's device file.
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
