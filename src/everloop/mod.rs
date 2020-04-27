mod led;
use crate::bus::memory_map::*;
use crate::bus::MatrixBus;
use heapless::Vec;
use typenum::Unsigned;
pub use led::Rgbw;

/// Bytes to set LED color as [R, G, B, W].
type LedBytes = [u8; 4];
/// Heapless capacity type large enough maximum number of possible LEDs across all devices.
type MaxLeds = heapless::consts::U35;
/// Heapless capacity type large enough for byte array of maximum possible LEDs and header bytes as passed to `MatrixBus` methods.
type MaxLedBytes = heapless::consts::U148;

const LED_BYTES: usize = core::mem::size_of::<LedBytes>();

fn _compile_time_checks() {
    // Need to enforce constraint that certain const/enum values correspond to heapless capacity type "values".
    // One way to accomplish this is to use both as the length of an array and assign them to eachother.  If the values 
    // differ, the arrays are different lengths and type, and Rust will trigger a compiler error.
    {
        // Require `MaxLeds` == `MATRIX_CREATOR_LEDS`
        type ValueArray = [u8; device_info::MATRIX_CREATOR_LEDS as usize];
        let _unused: ValueArray = [0u8; MaxLeds::USIZE];
    }
    {
        // Require `MaxLedBytes` has enough space to maximum possible LEDs and header bytes
        type ValueArray = [u8; device_info::MATRIX_CREATOR_LEDS as usize * LED_BYTES + crate::MATRIXBUS_HEADER_BYTES];
        let _unused: ValueArray = [0u8; MaxLedBytes::USIZE];
    }
}

/// Controls the ring of LEDS on a MATRIX device.
pub struct Everloop<'a> {
    bus: &'a dyn MatrixBus,
}

impl<'a> Everloop<'a> {
    /// Return an instance of Everloop.
    pub fn new(bus: &dyn MatrixBus) -> Everloop {
        Everloop { bus }
    }

    /// Map each `RGBW` to the respective MATRIX LED. LEDs not set are defaulted to black.
    ///
    /// # Example
    /// ```
    /// # let bus = matrix_rhal::Bus::init().unwrap();
    /// let everloop = matrix_rhal::Everloop::new(&bus);
    /// // Set 15 LEDs to blue and the remaining to black
    /// everloop.set(&vec![matrix_rhal::Rgbw::new(0,0,255,0); 15]);
    /// ```
    pub fn set(&self, leds: &[Rgbw]) {
        let device_leds = self.bus.device_leds() as usize;
        if leds.len() > device_leds as usize {
            panic!("Invalid LED set. This device only has {} LEDs", device_leds);
        }

        // create write buffer
        let led_bytes = device_leds * LED_BYTES;
        let mut request: Vec<u8, MaxLedBytes> = Vec::new();
        request.extend_from_slice(&(fpga_address::EVERLOOP as i32).to_ne_bytes()).unwrap();
        request.extend_from_slice(&(led_bytes as i32).to_ne_bytes()).unwrap(); // each LED RGBW requires 4 bytes

        // store all LED colors given
        for led in leds {
            request.extend_from_slice(&led.to_bytes()).unwrap();
        }
        // set remaining LEDs to black
        for _ in 0..(device_leds - leds.len()) {
            request.extend_from_slice(&Rgbw::black().to_bytes()).unwrap();
        }
        // render LEDs
        self.bus
            .write(&mut request);
    }

    /// Set all MATRIX LEDs to a single color
    pub fn set_all(&self, color: Rgbw) {
        let leds: Vec<Rgbw, MaxLeds> = core::iter::repeat(color).take(self.bus.device_leds() as usize).collect();
        self.set(&leds)
    }
}
