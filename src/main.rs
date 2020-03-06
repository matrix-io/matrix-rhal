// TODO: remove this in final release.
// This file is just meant to test things out.
use hal::gpio::config::*;
use matrix_rhal as hal;
use std::{thread, time};

fn main() {
    let bus = hal::Bus::init().unwrap();
    let sensors = hal::Sensors::new(&bus);
    let everloop = hal::Everloop::new(&bus);
    let gpio = hal::Gpio::new(&bus);

    everloop.set_all(hal::Rgbw::black());

    // test_gpio_set_value(&gpio);

    loop {
        println!("{:?}", gpio.get_states());

        // delay(100);
    }
}

fn test_gpio_set_value(gpio: &hal::Gpio) {
    gpio.set_configs(&[0, 1], Function::Digital).unwrap();
    gpio.set_config(1, Mode::Output).unwrap();
    gpio.set_config(0, Mode::Input).unwrap();

    gpio.set_config(1, State::On).unwrap();
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
