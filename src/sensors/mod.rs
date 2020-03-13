use crate::bus::{memory_map::*, MatrixBus};
use crate::Device;
mod data;
use data::*;

/// Communicates with the main sensors on the MATRIX Creator.
#[derive(Debug)]
pub struct Sensors<'a> {
    pub bus: &'a dyn MatrixBus,
}

// Read function for each sensor.
impl<'a> Sensors<'a> {
    /// Creates a new instance of Sensors.
    pub fn new(bus: &'a dyn MatrixBus) -> Sensors {
        if bus.get_device_name() != Device::Creator {
            panic!("Sensors are only available on the MATRIX Creator!")
        }

        Sensors { bus }
    }

    /// Return the latest UV sensor value.
    pub fn read_uv(&self) -> f32 {
        const BUFFER_LENGTH: usize = get_buffer_length(UV_BYTES);

        // create read buffer
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
        data[0] = (fpga_address::MCU + (mcu_offset::UV >> 1)) as i32;
        data[1] = UV_BYTES;

        // populate buffer
        self.bus
            .read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });

        data[2] as f32 / 1000.0
    }

    /// Return the latest Pressure sensor values.
    pub fn read_pressure(&self) -> Pressure {
        const BUFFER_LENGTH: usize = get_buffer_length(PRESSURE_BYTES);

        // create read buffer
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
        data[0] = (fpga_address::MCU + (mcu_offset::PRESSURE >> 1)) as i32;
        data[1] = PRESSURE_BYTES;

        // populate buffer
        self.bus
            .read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });

        Pressure {
            pressure: data[3] as f32 / 1000.0,
            altitude: data[2] as f32 / 1000.0,
            temperature: data[4] as f32 / 1000.0,
        }
    }

    /// Return the latest Humidity sensor values.
    pub fn read_humidity(&self) -> Humidity {
        const BUFFER_LENGTH: usize = get_buffer_length(HUMIDITY_BYTES);

        // create read buffer
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
        data[0] = (fpga_address::MCU + (mcu_offset::HUMIDITY >> 1)) as i32;
        data[1] = HUMIDITY_BYTES;

        // populate buffer
        self.bus
            .read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });

        Humidity {
            humidity: data[2] as f32 / 1000.0,
            temperature: data[3] as f32 / 1000.0,
        }
    }

    /// Return the latest IMU sensor values.
    pub fn read_imu(&self) -> Imu {
        const BUFFER_LENGTH: usize = get_buffer_length(IMU_BYTES);

        // create read buffer
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
        data[0] = (fpga_address::MCU + (mcu_offset::IMU >> 1)) as i32;
        data[1] = IMU_BYTES;

        // populate read buffer
        self.bus
            .read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });

        Imu {
            accel_x: data[2] as f32 / 1000.0,
            accel_y: data[3] as f32 / 1000.0,
            accel_z: data[4] as f32 / 1000.0,

            gyro_x: data[5] as f32 / 1000.0,
            gyro_y: data[6] as f32 / 1000.0,
            gyro_z: data[7] as f32 / 1000.0,

            mag_x: data[8] as f32 / 1000.0,
            mag_y: data[9] as f32 / 1000.0,
            mag_z: data[10] as f32 / 1000.0,

            // TODO: ask why we have these. They seem to be unused.
            mag_offset_x: data[11] as f32,
            mag_offset_y: data[12] as f32,
            mag_offset_z: data[13] as f32,

            // These values are already floats so we just need to treat them as one.
            yaw: f32::from_bits(data[14] as u32),
            pitch: f32::from_bits(data[15] as u32),
            roll: f32::from_bits(data[16] as u32),
        }
    }
}

/// Calculate the size a read buffer needs to be for a sensor.
///
/// Since all sensor's values are a byte each, we can divide it by 4 to see how many values need to be stored.
///
/// 2 is added to make room for the `address` and `byte_length` of `bus.read`.
const fn get_buffer_length(sensor_bytes: i32) -> usize {
    (sensor_bytes / 4 + 2) as usize
}
