pub mod kernel;
pub mod memory_map;
pub mod sensors;
use crate::error::Error;
use std::sync::Mutex;

/// Create, initialize, and return a MATRIX Bus
pub fn init<'a>() -> Result<kernel::Bus<'a>, Error> {
    let mut bus = kernel::Bus {
        device_file: "/dev/matrixio_regmap",
        rx_buffer: [0; 12288],
        tx_buffer: [0; 12288],
        regmap_fd: 0,
        usage: Mutex::new(()),
    };

    bus.init()?; // TODO: add question mark
    Ok(bus)
}

/// REMOVE THIS LATER. THIS IS JUST TO TEST FUNCTIONS
pub fn test(bus: &mut kernel::Bus) {
    // self.read(K_MCU_BASE_ADDRESS + (K_MEMORY_OFFSET_UV >> 1));
    bus.read(memory_map::K_CONF_BASE_ADDRESS, 8);

    // check if array changes
    for (i, &num) in bus.rx_buffer.into_iter().enumerate() {
        if num != 0 {
            println!("{}----->{}", i, num);
        }
    }

    // println!("{}", 0x05C344E8);
}
