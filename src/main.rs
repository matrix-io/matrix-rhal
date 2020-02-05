use matrix_rhal as hal;
use std::{thread, time};

fn main() {
    let mut bus = hal::bus::init().unwrap();
    println!("Device name: {:?}", bus.get_device_name());

    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();

    loop {
        // let x = bus.read_uv();
        // let x = bus.read_pressue();
        let x = bus.read_humidity();
        println!("{:?}", x);

        thread::sleep(ten_millis);
    }

    // hal::bus::test(&mut bus);
}
