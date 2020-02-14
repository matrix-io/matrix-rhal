use crate::bus::memory_map::*;
use crate::Bus;

pub struct Everloop<'a> {
    bus: &'a Bus,
}

impl<'a> Everloop<'a> {
    /// Return an instance of Everloop
    pub fn new(bus: &Bus) -> Everloop {
        Everloop { bus }
    }

    pub fn test(&self) {
        let buffer_length = self.bus.device_leds * 4;
        let mut image = Vec::new();

        image.push(fpga_address::EVERLOOP as i32);
        image.push(buffer_length as i32);

        for _ in 0..self.bus.device_leds {
            image.push(unsafe { std::mem::transmute::<[u8; 4], i32>([0, 0, 0, 0]) });
            //[r,g,b,w]
        }

        self.bus
            .write(unsafe { std::mem::transmute::<&mut Vec<i32>, &mut Vec<u8>>(&mut image) });
    }
}
