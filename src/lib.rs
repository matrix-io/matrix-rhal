extern crate spidev;
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::io;
use std::io::prelude::*;

pub const x: i32 = 33;
