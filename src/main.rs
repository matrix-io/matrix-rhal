use matrix_rhal as hal;
use std::sync::Mutex;

fn main() {
    let mut bus = hal::bus::new();
    bus.init();
    bus.read_uv();
}
