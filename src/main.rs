// TODO: remove this in final release.
// This file is just meant to test things out.
use matrix_rhal as hal;
use std::{thread, time};

fn main() {
    let bus = hal::Bus::init().unwrap();
    let mut sensors = hal::Sensors::new(&bus);
    let mut everloop = hal::Everloop::new(&bus);

    everloop.set_all(hal::Rgbw::new(0, 0, 0, 0));

    loop {
        println!("--> {:#?}", sensors.read_uv());
        println!("--> {:#?}", sensors.read_pressure());
        println!("--> {:#?}", sensors.read_humidity());
        println!("-->{:#?}", sensors.read_imu());

        // let x = sensors.read_imu();
        // println!("yaw -> {}", x.yaw);
        // println!("pitch -> {}", x.pitch);
        // println!("roll -> {}", x.roll);

        // delay
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }
}
