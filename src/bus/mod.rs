pub mod memory_map;
use memory_map::*;
// pub mod sensors;
use crate::{error::Error, Device, Sensors};
use nix::fcntl::{open, OFlag}; // https://linux.die.net/man/3/open
use nix::ioctl_read_bad;
use nix::sys::stat::Mode;
use nix::unistd::close;
use std::sync::Mutex; // https://linux.die.net/man/2/close

// Generate read() function
ioctl_read_bad!(read, RD_VALUE, [i32]);

// Generate write function
// TODO:

/// The bridge for talking to the MATRIX Kernel Modules.
/// Most, if not all, MATRIX functionality requires this Bus to read and write data.
pub struct Bus {
    /// Path for the device file being used. This is what's used to communicate with the MATRIX Kernel.
    pub device_file: &'static str,
    /// Read buffer passed into IOCTL read function.
    pub rx_buffer: [i32; 12288],
    /// Write buffer passed into IOCTL read function.
    pub tx_buffer: [i32; 12288],
    /// File descriptor for kernel abstraction.
    pub regmap_fd: std::os::unix::io::RawFd,
    /// Mutex to prevent collisions during read/write.
    pub usage: Mutex<()>,
    /// The type of MATRIX device that's currently attached.
    pub device_name: Device,
    /// Number of LEDS on the MATRIX device.
    pub device_leds: u8,
    /// Requests and stores sensor data.
    pub sensors: Sensors,
}

impl Bus {
    // pub fn read

    /// Create, initialize, and return a MATRIX Bus
    pub fn init() -> Result<Bus, Error> {
        let mut bus = Bus {
            device_file: "/dev/matrixio_regmap",
            rx_buffer: [0; 12288],
            tx_buffer: [0; 12288],
            regmap_fd: 0,
            usage: Mutex::new(()),
            device_name: Device::Unknown,
            device_leds: 0,
            sensors: Sensors::default(),
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

    pub fn write(&self, add: u16, data: &u8, length: i32) -> bool {
        todo!();
    }

    /// Populate the Bus' rx_buffer with the requested data.
    /// Any data returned will start at the index 2 of the buffer.
    // TODO: manually pass in a buffer. Avoid using rx_buffer
    pub fn read(&mut self, address: u16, bytes: i32) {
        self.usage.lock();

        // set request and bytes needed
        self.rx_buffer[0] = address as i32;
        self.rx_buffer[1] = bytes;

        // IOCTL read call
        unsafe {
            read(self.regmap_fd, &mut self.rx_buffer).unwrap();
        }
    }

    /// Close the file descriptor that's communicating with the MATRIX Kernel's device file.
    pub fn close(&self) {
        self.usage.lock();
        close(self.regmap_fd).unwrap();
    }

    /// Return the type of MATRIX device being used.
    fn get_device_name(&mut self) -> Result<Device, Error> {
        self.usage.lock();

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

// /// REMOVE THIS LATER. THIS IS JUST TO TEST FUNCTIONS
// pub fn test(bus: &mut kernel::Bus) {
//     // self.read(K_MCU_BASE_ADDRESS + (K_MEMORY_OFFSET_UV >> 1));
//     bus.read(memory_map::K_CONF_BASE_ADDRESS, 8);

//     // check if array changes
//     for (i, &num) in bus.rx_buffer.into_iter().enumerate() {
//         if num != 0 {
//             println!("{}----->{}", i, num);
//         }
//     }

//     // println!("{}", 0x05C344E8);
// }
