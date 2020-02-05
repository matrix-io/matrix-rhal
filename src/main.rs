use matrix_rhal as hal;
use std::{thread, time};

fn main() {
    let mut bus = hal::bus::init().unwrap();

    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();

    println!(
        "The MATRIX {:?} has {} LEDs!",
        bus.device_name, bus.device_leds
    );
    // loop {
    //     // let x = bus.read_uv();
    //     // let x = bus.read_pressue();
    //     // let x = bus.read_humidity();
    //     let x = bus.read_imu();
    //     println!("{:#?}", x);

    //     thread::sleep(ten_millis);
    // }

    // hal::bus::test(&mut bus);
}
