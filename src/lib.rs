#![cfg_attr(not(feature = "std"), no_std)]

pub mod bus;
mod error;
mod everloop;
pub mod gpio;
pub mod info;
mod sensors;

pub use error::Error;
pub use everloop::Everloop;
pub use everloop::Rgbw;
pub use gpio::Gpio;
pub use sensors::Sensors;

#[macro_export]
macro_rules! with_std { ($($i:item)*) => ($(#[cfg(feature = "std")]$i)*) }
#[macro_export]
macro_rules! without_std { ($($i:item)*) => ($(#[cfg(not(feature = "std"))]$i)*) }

/// Buffers passed to `impl MatrixBus` methods contain:
/// | Bytes | |
/// |-|-|
/// | 0-3 | Address for SPI operation
/// | 4-7 | Size of data
/// | 8.. | Data
const MATRIXBUS_HEADER_BYTES: usize = core::mem::size_of::<i32>() * 2;

/// The Different types of MATRIX Devices
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", non_exhaustive)] // Xtensa support currently limited to Rust 1.37
pub enum Device {
    /// MATRIX Creator.
    Creator,
    /// MATRIX Voice.
    Voice,
    /// Placeholder until the device is known.
    Unknown,
}
