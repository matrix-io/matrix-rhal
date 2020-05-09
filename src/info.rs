use crate::bus::{memory_map::*, MatrixBus};
use crate::{as_mut_u8_slice, Device, Error};

/// Return the type of MATRIX device being used and the version of the board.
pub fn get_device_info(bus: &dyn MatrixBus) -> Result<(Device, u32), Error> {
    // create read buffer
    let mut data = [0i32; 2]; // device_name(4 bytes) device_version(4 bytes)

    bus.read(fpga_address::CONF, as_mut_u8_slice(&mut data));
    let device_name = data[0];
    let device_version = data[1];

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
    let mut data = [0u16; 2]; // value0(2 bytes) value1(2bytes) // TODO: ask what these values represent
    bus.read(fpga_address::CONF + 4, as_mut_u8_slice(&mut data));

    let value0 = data[0] as u32; // store 1st 16 bits
    let value1 = data[1] as u32; // store 2nd 16 bits
    let frequency = (device_info::FPGA_CLOCK * value0) / value1;

    Ok(frequency)
}
