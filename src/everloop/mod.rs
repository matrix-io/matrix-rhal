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
        let mut image = Vec::<u8>::with_capacity(buffer_length as usize);

        // space for address and buffer length / TODO: REMOVE

        for _ in 0..35 {
            image.push(0);
            image.push(0);
            image.push(1);
            image.push(0);
        }

        self.bus
            .write(fpga_address::EVERLOOP, &mut image[..], buffer_length as i32);

        // println!("{:?}", image);
        // println!("---> {}", image[..].len());
    }
}
