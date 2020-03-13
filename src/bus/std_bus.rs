use crate::{bus::memory_map::*, error::Error, Device};
use super::BusImpl;
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
        let (name, version) = bus.get_device_info()?;
        bus.device_name = name;
        bus.device_version = version;

        bus.device_leds = match bus.device_name {
            Device::Creator => device_info::MATRIX_CREATOR_LEDS,
            Device::Voice => device_info::MATRIX_VOICE_LEDS,
            _ => panic!("Cannot determine number of LEDs on device (This is a hard-coded value)."),
        };
        bus.fpga_frequency = bus.get_fpga_frequency()?;

        Ok(bus)
    }

    pub fn device_name(&self) -> Device {
        self.device_name
    }
    pub fn device_version(&self) -> u32 {
        self.device_version
    }
    pub fn device_leds(&self) -> u8 {
        self.device_leds
    }
    pub fn fpga_frequency(&self) -> u32 {
        self.fpga_frequency
    }

    /// Send a write buffer to the MATRIX Kernel Modules. The buffer requires an `address` to request,
    /// the `byte_length` of the data being given, and then the rest of the data itself.
    ///
    /// # Usage
    ///  ```
    ///  let bus = matrix_rhal::Bus::init().unwrap();
    ///
    ///  # let address_offset = 0;
    ///  let some_value: u16 = 237;
    ///  let mut buffer: [u32; 3] = [0; 3];
    ///     
    ///  // address to query
    ///  buffer[0] = (matrix_rhal::bus::memory_map::fpga_address::GPIO + address_offset) as u32;
    ///  // byte length of data (u16 = 2 bytes)
    ///  buffer[1] = 2;
    ///  // data being sent
    ///  buffer[2] = some_value as u32;
    ///
    ///  // send buffer
    ///  bus.write(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(&mut buffer) });
    ///  ```
    pub fn write(&self, write_buffer: &mut [u8]) {
        unsafe {
            // TODO: error handling. Not sure if an error here would be worth recovering from.
            ioctl_write(self.regmap_fd, write_buffer).expect("error in IOCTL WRITE");
        }
    }

    /// Send a read buffer to the MATRIX Kernel Modules. The buffer requires an `address` to request and
    /// the `byte_length` of what's expected to be returned. Once sent, the buffer return with populated
    /// data.
    ///
    /// Keep in mind, the returned buffer will still have the `address` and `byte_length` that was passed.
    ///
    /// # Usage
    ///  ```
    ///  let bus = matrix_rhal::Bus::init().unwrap();
    ///  let mut buffer: [u32; 4] = [0; 4];
    ///
    ///  // address to query
    ///  buffer[0] = (matrix_rhal::bus::memory_map::fpga_address::CONF) as u32;
    ///  // bytes being requested
    ///  buffer[1] = 8;
    ///
    ///  // populate buffer
    ///  bus.read(unsafe { std::mem::transmute::<&mut [u32], &mut [u8]>(&mut buffer) });
    ///
    ///  // returned data will start at buffer[2]
    ///  println!("{:?}", buffer);
    ///  ```
    pub fn read(&self, read_buffer: &mut [u8]) {
        unsafe {
            // TODO: error handling. Not sure if an error here would be worth recovering from.
            ioctl_read(self.regmap_fd, read_buffer).expect("error in IOCTL READ");
        }
    }

    /// Close the file descriptor that's communicating with the MATRIX Kernel's device file.
    pub fn close(&self) {
        close(self.regmap_fd).unwrap();
    }

    /// Return the type of MATRIX device being used and the version of the board.
    fn get_device_info(&self) -> Result<(Device, u32), Error> {
        // create read buffer
        let mut data: [i32; 4] = [0; 4];
        data[0] = fpga_address::CONF as i32;
        data[1] = 8; // device_name(4 bytes) device_version(4 bytes)

        self.read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });
        let device_name = data[2];
        let device_version = data[3];

        Ok((
            match device_name {
                device_info::MATRIX_CREATOR => Device::Creator,
                device_info::MATRIX_VOICE => Device::Voice,
                _ => return Err(Error::UnknownDevice),
            },
            device_version as u32,
        ))
    }

    /// Updates the Bus to have the last known FPGA frequency of the MATRIX device.
    fn get_fpga_frequency(&self) -> Result<u32, Error> {
        // create read buffer
        let mut data: [i32; 3] = [0; 3];
        data[0] = (fpga_address::CONF + 4) as i32;
        data[1] = 4; // value0(2 bytes) value1(2bytes) // TODO: ask what these values represent

        self.read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });

        // extract both u16 numbers from u32
        let value0 = data[2] >> 16; // store 1st 16 bits
        let value1 = !(value0 << 16) & data[2]; // store 2nd 16 bits
        let frequency = (device_info::FPGA_CLOCK * value0 as u32) / value1 as u32;

        Ok(frequency)
    }
}
