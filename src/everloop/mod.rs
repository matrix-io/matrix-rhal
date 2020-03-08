mod led;
use crate::bus::memory_map::*;
use crate::Bus;
pub use led::Rgbw;

/// Controls the ring of LEDS on a MATRIX device.
#[derive(Debug)]
pub struct Everloop<'a> {
    bus: &'a Bus,
}

impl<'a> Everloop<'a> {
    /// Return an instance of Everloop.
    pub fn new(bus: &Bus) -> Everloop {
        Everloop { bus }
    }

    /// Map each `RGBW` to the respective MATRIX LED. LEDs not set are defaulted to black.
    ///
    /// # Example
    /// ```
    /// // Set 15 LEDs to blue and the remaining to black
    /// everloop.set(&vec![hal::Rgbw::new(0,0,255,0); 15]);
    /// ```
    pub fn set(&self, leds: &[Rgbw]) {
        if leds.len() > self.bus.device_leds as usize {
            panic!(
                "Invalid LED set. This device only has {} LEDs",
                self.bus.device_leds
            );
        }

        // create write buffer
        let mut request = Vec::with_capacity(self.bus.device_leds as usize + 2);
        request.push(fpga_address::EVERLOOP as i32);
        request.push((self.bus.device_leds * 4) as i32); // each LED RGBW requires 4 bytes

        // store all LED colors given
        for led in leds {
            request
                .push(unsafe { std::mem::transmute::<[u8; 4], i32>([led.r, led.g, led.b, led.w]) });
        }

        // set remaining LEDs to black
        for _ in 0..(request.capacity() - request.len()) {
            request.push(unsafe { std::mem::transmute::<[u8; 4], i32>([0, 0, 0, 0]) })
        }

        // render LEDs
        self.bus
            .write(unsafe { std::mem::transmute::<&mut Vec<i32>, &mut Vec<u8>>(&mut request) });
    }

    /// Set all MATRIX LEDs to a single color
    pub fn set_all(&self, color: Rgbw) {
        let mut leds = Vec::new();
        leds.extend(std::iter::repeat(color).take(self.bus.device_leds as usize));

        self.set(&leds)
    }
}
