use super::MatrixBus;
use core::convert::TryFrom;
pub mod bus;
pub mod error;

pub fn init() -> impl MatrixBus {
    bus::Bus::init().unwrap()
}

/// Converts `i32` returned by ESP-IDF native functions into `Result`
fn esp_int_into_result(value: i32) -> Result<(), crate::Error> {
    if value == 0 {
        Ok(())
    } else if let Ok(error) = error::EspError::try_from(value) {
        Err(crate::Error::EspIdf { error })
    } else {
        Err(crate::Error::EnumFromIntError {
            value: value as u32,
        })
    }
}
