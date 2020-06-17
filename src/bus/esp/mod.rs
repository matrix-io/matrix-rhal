use super::MatrixBus;
pub mod bus;

pub fn init() -> impl MatrixBus {
    bus::Bus::init().unwrap()
}
