pub mod memory_map;
use crate::{error::Error, Device, Sensors};
use memory_map::*;
use nix::fcntl::{open, OFlag}; // https://linux.die.net/man/3/open
use nix::sys::stat::Mode;
use nix::unistd::close;
use nix::{ioctl_read_bad, ioctl_write_ptr_bad};
use std::sync::Mutex; // https://linux.die.net/man/2/close

// Generate read() function
ioctl_read_bad!(read, ioctl_code::READ, [i32]);

// Generate write function
ioctl_write_ptr_bad!(write, ioctl_code::WRITE, [u8]);

/// The bridge for talking to the MATRIX Kernel Modules.
/// Most, if not all, MATRIX functionality requires this Bus to read and write data.
pub struct Bus {
    /// Path for the device file being used. This is what's used to communicate with the MATRIX Kernel.
    pub device_file: &'static str,
    /// File descriptor for kernel abstraction.
    pub regmap_fd: std::os::unix::io::RawFd,
    /// The type of MATRIX device that's currently attached.
    pub device_name: Device,
    /// Number of LEDS on the MATRIX device.
    pub device_leds: u8,
    /// Read/Write lock for IOCTL calls.
    pub usage: Mutex<()>,
}

impl Bus {
    /// Create, initialize, and return a MATRIX Bus
    pub fn init() -> Result<Bus, Error> {
        let mut bus = Bus {
            device_file: "/dev/matrixio_regmap",
            regmap_fd: 0,
            device_name: Device::Unknown,
            device_leds: 0,
            usage: Mutex::new(()),
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

        Ok(bus)
    }

    /// Populate a buffer with the requested data.
    /// `address` and `read_buffer` will be added to the 1 and 2 index of your buffer.
    /// Any data added should start at index 2.
    pub fn write(&self, address: u16, write_buffer: &mut [u8], bytes: i32) {
        // TODO: FINISH WHERE YOU LEFT OFF
        self.usage.lock().unwrap();

        // IOCTL write call
        unsafe {
            let new_buffer = std::mem::transmute::<&mut [u8], &mut [i32]>(write_buffer);

            // new_buffer[0] = address as i32; // stores the address we'll send information to
            // new_buffer[1] = bytes; // and the amount of bytes our request needs.

            let mut x = Vec::<i32>::new();
            x.push(address as i32);
            x.push(bytes);
            x.extend(new_buffer.to_vec());
            let y = std::mem::transmute::<&mut [i32], &mut [u8]>(&mut x);

            write(self.regmap_fd, &y[..]).expect("error in IOCTL WRITE");
        }
    }

    /// Populate a buffer with the requested data.
    /// `address` and `read_buffer` will be added to the 1 and 2 index of your buffer.
    /// Any data returned will start at the index 2.
    pub fn read(&self, address: u16, read_buffer: &mut [i32], bytes: i32) {
        self.usage.lock().unwrap();

        // set the address we'll send information to
        read_buffer[0] = address as i32;
        // amount of bytes to store
        read_buffer[1] = bytes;

        // IOCTL read call
        unsafe {
            read(self.regmap_fd, read_buffer).unwrap();
        }
    }

    /// Close the file descriptor that's communicating with the MATRIX Kernel's device file.
    pub fn close(&self) {
        close(self.regmap_fd).unwrap();
    }

    /// Return the type of MATRIX device being used.
    fn get_device_name(&self) -> Result<Device, Error> {
        let mut data: [i32; 4] = [0; 4];

        // store the bytes representing device type & version
        self.read(fpga_address::CONF as u16, &mut data, 8);

        let device_name = data[2];
        // let device_version = self.rx_buffer[3]; // currently unused

        match device_name {
            device_info::MATRIX_CREATOR => Ok(Device::Creator),
            device_info::MATRIX_VOICE => Ok(Device::Voice),
            _ => Err(Error::UnknownDevice),
        }
    }
}
