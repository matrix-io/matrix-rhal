mod memory_map;
use memory_map::*;
use nix::fcntl::{open, OFlag}; // https://linux.die.net/man/3/open
use nix::sys::stat::Mode;
use nix::unistd::close; // https://linux.die.net/man/2/close
use nix::*;
use std::sync::Mutex;

const kFPGAClock: u32 = 50000000; // Physical OSC = 50MHz
const K_MATRIX_CREATOR_N_LEDS: i32 = 35;
const K_MATRIX_VOICE_N_LEDS: i32 = 18;
const K_MATRIX_CREATOR: i32 = 0x05C344E8;
const K_MATRIX_VOICE: i32 = 0x6032BAD2;

const WR_VALUE: i32 = 1200;
const RD_VALUE: i32 = 1200;

// kernel bus
pub struct Bus<'a> {
    /// Path for the device file being used. This is what's used to communicate with the MATRIX Kernel.
    pub device_file: &'a str,
    pub rx_buffer: [i32; 12288],
    pub tx_buffer: [i32; 12288],
    /// File descriptor for kernel abstraction.
    pub regmap_fd: i32, // change type to https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
    // Empty because we don't need to pass any data (yet)
    pub usage: Mutex<()>,
}

impl<'a> Bus<'a> {
    pub fn init(&mut self) -> bool {
        self.usage.lock().unwrap();

        // Open MATRIX Kernel's device file & retrieve file descriptor id.
        self.regmap_fd = open(self.device_file, OFlag::O_RDWR, Mode::empty()).unwrap();

        true
    }

    pub fn write(&self, add: u16, data: &u8, length: i32) -> bool {
        todo!();
    }

    pub fn read(&mut self, add: u16, data: &mut u8, length: i32) -> bool {
        self.usage.lock();

        self.rx_buffer[0] = add as i32;
        self.rx_buffer[1] = length;

        // TODO: test
        ioctl_read_bad!(read, RD_VALUE, i32);
        unsafe {
            let x = read(self.regmap_fd, &mut self.rx_buffer[2]);
            println!("--> {:?}", x);
        }

        todo!();
    }

    /// Close the file descriptor that's communicating with the MATRIX Kernel's device file.
    pub fn close(&self) {
        close(self.regmap_fd).unwrap();
    }

    pub fn read_uv(&mut self) {
        let uv_data: f32;
        let data_size = std::mem::size_of::<f32>() as i32;

        self.read(
            K_MCU_BASE_ADDRESS + (K_MEMORY_OFFSET_UV >> 1),
            , // TODO: find cleaner way to extract data
            &mut data_size,
        );

        todo!();
    }
}

pub fn new<'a>() -> Bus<'a> {
    Bus {
        device_file: "/dev/matrixio_regmap",
        rx_buffer: [0; 12288],
        tx_buffer: [0; 12288],
        regmap_fd: 0,
        usage: Mutex::new(()),
    }
}
