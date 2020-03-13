use crate::bus::MatrixBus;
use crate::{bus::memory_map::*, Device, Error};

/// Return the type of MATRIX device being used and the version of the board.
pub fn get_device_info(bus: &dyn MatrixBus) -> Result<(Device, u32), Error> {
    // create read buffer
    let mut data: [i32; 4] = [0; 4];
    data[0] = fpga_address::CONF as i32;
    data[1] = 8; // device_name(4 bytes) device_version(4 bytes)

    bus.read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });
    let device_name = data[2];
    let device_version = data[3];

    Ok((
        match device_name {
            device_info::MATRIX_CREATOR => Device::Creator,
            device_info::MATRIX_VOICE => Device::Voice,
            _ => return Err(Error::UnknownDevice),
        },
        device_version as u32,
    ))
}

/// Updates the Bus to have the last known FPGA frequency of the MATRIX device.
pub fn get_fpga_frequency(bus: &dyn MatrixBus) -> Result<u32, Error> {
    // create read buffer
    let mut data: [i32; 3] = [0; 3];
    data[0] = (fpga_address::CONF + 4) as i32;
    data[1] = 4; // value0(2 bytes) value1(2bytes) // TODO: ask what these values represent

    bus.read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });

    // extract both u16 numbers from u32
    let value0 = data[2] >> 16; // store 1st 16 bits
    let value1 = !(value0 << 16) & data[2]; // store 2nd 16 bits
    let frequency = (device_info::FPGA_CLOCK * value0 as u32) / value1 as u32;

    Ok(frequency)
}
