mod led;
use crate::bus::{memory_map::*, MatrixBus};
pub use led::Rgbw;

/// Controls the ring of LEDS on a MATRIX device.
#[derive(Debug)]
pub struct Everloop {
    bus: &'static dyn MatrixBus,
}

impl Everloop {
    /// Return an instance of Everloop.
    pub fn new(bus: &'static dyn MatrixBus) -> Everloop {
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
        let device_leds = self.bus.get_device_leds();

        if leds.len() > device_leds as usize {
            panic!("Invalid LED set. This device only has {} LEDs", device_leds);
        }

        // create write buffer
        let mut request = Vec::with_capacity(device_leds as usize + 2);
        request.push(fpga_address::EVERLOOP as i32);
        request.push((device_leds * 4) as i32); // each LED RGBW requires 4 bytes

        // store all LED colors given
        for led in leds {
            request.push(led.as_bytes());
        }

        // set remaining LEDs to black
        for _ in 0..(request.capacity() - request.len()) {
            request.push(Rgbw::black().as_bytes())
        }

        // render LEDs
        self.bus
            .write(unsafe { std::mem::transmute::<&mut Vec<i32>, &mut Vec<u8>>(&mut request) });
    }

    /// Set all MATRIX LEDs to a single color
    pub fn set_all(&self, color: Rgbw) {
        let device_leds = self.bus.get_device_leds() as usize;

        let mut leds = Vec::new();
        leds.extend(std::iter::repeat(color).take(device_leds));

        self.set(&leds)
    }
}
