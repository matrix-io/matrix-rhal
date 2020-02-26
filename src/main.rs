// TODO: remove this in final release.
// This file is just meant to test things out.
use matrix_rhal as hal;
use std::{thread, time};

fn main() {
    let bus = hal::Bus::init().unwrap();
    let sensors = hal::Sensors::new(&bus);
    let everloop = hal::Everloop::new(&bus);

    everloop.set_all(hal::Rgbw::new(0, 0, 0, 0));

    // loop {
    test_sensors(&sensors);
    delay(100);
    // }
}

fn test_sensors(sensors: &hal::Sensors) {
    println!("--> {:#?}", sensors.read_uv());
    println!("--> {:#?}", sensors.read_pressure());
    println!("--> {:#?}", sensors.read_humidity());
    println!("--> {:#?}", sensors.read_imu());
}

fn delay(ms: u64) {
    let ten_millis = time::Duration::from_millis(ms);
    thread::sleep(ten_millis);
}
