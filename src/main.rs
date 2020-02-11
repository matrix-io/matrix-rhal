use matrix_rhal as hal;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, time};

fn main() {
    let mut bus = hal::Bus::init().unwrap();
    let mut sensors = hal::Sensors::new(&bus);

    loop {
        sensors.read_uv();
        sensors.read_pressure();
        sensors.read_humidity();
        sensors.read_imu();

        println!("--> {:#?}", sensors.uv);
        println!("--> {:#?}", sensors.pressure);
        println!("--> {:#?}", sensors.humidity);
        println!("--> {:#?}", sensors.imu);

        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }
}
