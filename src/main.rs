// TODO: remove this in final release.
// This file is just meant to test things out.
use hal::gpio::config::*;
use matrix_rhal as hal;
// use std::{thread, time};

fn main() {
    let bus = hal::bus::init();
    let everloop = hal::Everloop::new(&*bus);

    everloop.set_all(hal::Rgbw::new(0, 0, 0, 0));
}

fn delay(ms: u64) {
    // let ten_millis = time::Duration::from_millis(ms);
    // thread::sleep(ten_millis);
}
