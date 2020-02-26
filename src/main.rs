// TODO: remove this in final release.
// This file is just meant to test things out.
use matrix_rhal as hal;
use std::{thread, time};

fn main() {
    let bus = hal::Bus::init().unwrap();
    let sensors = hal::Sensors::new(&bus);
    let everloop = hal::Everloop::new(&bus);
    let gpio = hal::Gpio::new(&bus);

    everloop.set_all(hal::Rgbw::new(0, 0, 0, 0));

    gpio.set_mode(0, hal::Mode::Input);
    gpio.set_function(0, hal::Function::Digital);

    gpio.set_mode(1, hal::Mode::Output);
    gpio.set_function(1, hal::Function::Digital);
    gpio.set_value(1, 1);

    loop {
        println!("{:?}", gpio.get_value(0));

        // test_gpio_get_value(&gpio);
        // test_sensors(&sensors);
        delay(50);
    }
}

fn test_gpio_get_value(gpio: &hal::Gpio) {
    for i in 0..16 {
        gpio.set_mode(i, hal::Mode::Input);
    }

    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        gpio.get_value(0),
        gpio.get_value(1),
        gpio.get_value(2),
        gpio.get_value(3),
        gpio.get_value(4),
        gpio.get_value(5),
        gpio.get_value(6),
        gpio.get_value(7),
        gpio.get_value(8),
        gpio.get_value(9),
        gpio.get_value(10),
        gpio.get_value(11),
        gpio.get_value(12),
        gpio.get_value(13),
        gpio.get_value(14),
        gpio.get_value(15),
    );
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
