// TODO: remove this in final release.
// This file is just meant to test things out.
// use hal::gpio::config::*;
use matrix_rhal as hal;
use std::time::{Duration, Instant};
use std::{thread, time};

fn main() {
    let now = Instant::now();
    let bus = hal::bus::init().unwrap();
    println!("Bus Created: {:#?}", now.elapsed());

    let sensors = hal::Sensors::new(&bus);

    for _ in 0..1000 {
        // println!("{:?}", gpio.get_states());
        test_sensors(&sensors);
        // delay(100);
    }
    println!("Sensor Reads: {:#?}", now.elapsed());
}

// fn test_gpio_set_value(gpio: &hal::Gpio) {
//     gpio.set_configs(&[0, 1], Function::Digital).unwrap();
//     gpio.set_config(1, Mode::Output).unwrap();
//     gpio.set_config(0, Mode::Input).unwrap();
//     gpio.set_config(1, State::On).unwrap();
// }

// fn test_gpio_pwm(gpio: &hal::Gpio) {
//     gpio.set_config(2, Function::Pwm).unwrap();
//     gpio.set_config(2, Mode::Output).unwrap();
//     gpio.set_pwm(2, 50.0, 50.0).unwrap();
// }

// fn test_gpio_set_servo(gpio: &hal::Gpio) {
//     gpio.set_config(3, Function::Pwm).unwrap();
//     gpio.set_config(3, Mode::Output).unwrap();

//     gpio.set_servo_angle(3, 0, 0.7).unwrap();
//     delay(2000);
//     gpio.set_servo_angle(3, 180, 0.7).unwrap();
// }

fn test_sensors(sensors: &hal::Sensors) {
    // println!("--> {:#?}", sensors.read_uv());
    // println!("--> {:#?}", sensors.read_pressure());
    // println!("--> {:#?}", sensors.read_humidity());
    // println!("--> {:#?}", sensors.read_imu());

    sensors.read_uv();
    sensors.read_pressure();
    sensors.read_humidity();
    sensors.read_imu();
}

fn delay(ms: u64) {
    let ten_millis = time::Duration::from_millis(ms);
    thread::sleep(ten_millis);
}
