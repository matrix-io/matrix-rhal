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

fn as_slice<'a, A, B>(orig: &[A]) -> &'a [B] {
    unsafe {
        use core::mem::size_of;
        core::slice::from_raw_parts(
            orig.as_ptr() as *const _,
            orig.len() * size_of::<A>() / size_of::<B>(),
        )
    }
}

fn as_mut_slice<'a, A, B>(orig: &mut [A]) -> &'a mut [B] {
    unsafe {
        use core::mem::size_of;
        core::slice::from_raw_parts_mut(
            orig.as_ptr() as *mut _,
            orig.len() * size_of::<A>() / size_of::<B>(),
        )
    }
}

fn as_u8_slice<'a, A>(orig: &[A]) -> &'a [u8] {
    as_slice(orig)
}

fn as_mut_u8_slice<'a, A>(orig: &mut [A]) -> &'a mut [u8] {
    as_mut_slice(orig)
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of_val;
    #[test]
    fn as_slice() {
        let i32_array = [0i32; 4];
        let u16_array = [0u16; 4];
        assert_eq!(as_u8_slice(&i32_array).len(), size_of_val(&i32_array));
        assert_eq!(as_u8_slice(&u16_array).len(), size_of_val(&u16_array));
    }
}
