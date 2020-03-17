mod led;
use crate::bus::memory_map::*;
use crate::bus::MatrixBus;
use heapless::{consts::U144 as MAX_LED_BYTES, Vec};
pub use led::Rgbw;

/// Bytes to set LED color as [R, G, B, W].
type LedBytes = [u8; 4];

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
        let led_bytes = device_leds * core::mem::size_of::<LedBytes>();
        let mut request: Vec<u8, MAX_LED_BYTES> = Vec::new();
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
        // The array length, `N` must be equal to largest number of LEDs.  In this case MATRIX Creator
        let leds: Vec<Rgbw, heapless::consts::U35> = core::iter::repeat(color).take(self.bus.device_leds() as usize).collect();
        self.set(&leds)
    }
}
