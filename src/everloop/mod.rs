mod led;
use crate::bus::memory_map::*;
use crate::Bus;
pub use led::Rgbw;

pub struct Everloop<'a> {
    bus: &'a Bus,
}

impl<'a> Everloop<'a> {
    /// Return an instance of Everloop
    pub fn new(bus: &Bus) -> Everloop {
        Everloop { bus }
    }

    pub fn set(&self, leds: &Vec<Rgbw>) {
        // create write buffer
        let mut request = Vec::with_capacity(leds.len() + 2);
        // everloop address
        request.push(fpga_address::EVERLOOP as i32);
        // byte length of request in write_buffer
        request.push((self.bus.device_leds * 4) as i32);

        // store all LED colors given
        for led in leds {
            request
                .push(unsafe { std::mem::transmute::<[u8; 4], i32>([led.r, led.g, led.b, led.w]) });
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
