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

    let mut toggle = 0;
    loop {
        println!("counter: {}, {:?}", toggle, gpio.get_values());

        if toggle > 50 {
            gpio.set_config(1, State::On);
        } else if toggle < 50 {
            gpio.set_config(1, State::Off);
        }

        if toggle > 100 {
            toggle = 0
        }

        toggle += 2;

        // test_sensors(&sensors);
        delay(100);
    }
}

fn test_gpio_set_value(gpio: &hal::Gpio) {
    // TODO: If pinA is configured before pinB and pinA position > pinB position, there are issues with State & Output config.
    gpio.set_configs(&[0, 1], Function::Digital);

    gpio.set_config(1, Mode::Output);
    gpio.set_config(0, Mode::Input);

    // set pin 0 to receive a signal
    // gpio.set_config(0, Mode::Input);
    // gpio.set_config(0, Function::Digital);

    // // set pin 1 to output a signal
    // gpio.set_config(1, Mode::Output);
    // gpio.set_config(1, Function::Digital);
    // gpio.set_config(1, State::On);
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
