use matrix_rhal as hal;
use std::sync::Mutex;

fn main() {
    let mut bus = hal::bus::init();
    println!("Device name: {:?}", bus.get_device_name());

    // hal::bus::test(&mut bus);
}
