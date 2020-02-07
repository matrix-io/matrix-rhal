use matrix_rhal as hal;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, time};

fn main() {
    let bus = hal::Bus::init().unwrap();
    let bus = Arc::new(Mutex::new(bus));

    for i in 0..30 {
        let mutex = bus.clone();

        thread::spawn(move || {
            let mut bus = mutex.lock().unwrap();

            bus.read_uv();
            // bus.sensors.read.uv(); // TODO
            println!("{:?}", bus.sensors.uv.uv);
        });
    }

    // let ten_millis = time::Duration::from_millis(10);
    // let now = time::Instant::now();

    // println!(
    //     "The MATRIX {:?} has {} LEDs!",
    //     bus.device_name, bus.device_leds
    // );
}
