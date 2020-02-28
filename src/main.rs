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

    everloop.set_all(hal::Rgbw::new(0, 0, 0, 0));
    test_gpio_set_value(&gpio);

    loop {
        test_gpio_get_value(&gpio);
        // test_sensors(&sensors);
        delay(50);
    }
}

fn test_gpio_set_value(gpio: &hal::Gpio) {
    // set pin 0 to receive a signal
    gpio.set_mode(0, Mode::Input);
    gpio.set_function(0, Function::Digital);

    // set pin 1 to output a signal
    gpio.set_mode(1, Mode::Output);
    gpio.set_function(1, Function::Digital);
    gpio.set_value(1, State::On);
}

fn test_gpio_get_value(gpio: &hal::Gpio) {
    for i in 0..16 {
        // gpio.set_mode(i, Mode::Input);
    }

    println!("{:?}", gpio.get_values());
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
