use crate::bus::{memory_map::*, MatrixBus};
use crate::{as_mut_bytes, Device};
mod data;
use data::*;

/// Communicates with the main sensors on the MATRIX Creator.
pub struct Sensors<'a> {
    pub bus: &'a dyn MatrixBus,
}

// Read function for each sensor.
impl<'a> Sensors<'a> {
    /// Creates a new instance of Sensors.
    pub fn new(bus: &dyn MatrixBus) -> Sensors {
        if bus.device_name() != Device::Creator {
            panic!("Sensors are only available on the MATRIX Creator!")
        }

        Sensors { bus }
    }

    /// Return the latest UV sensor value.
    pub fn read_uv(&self) -> f32 {
        // create read buffer
        let mut data = [0i32; get_buffer_length(UV_BYTES)];
        let address = fpga_address::MCU + (mcu_offset::UV >> 1);
        // populate buffer
        self.bus.read(address, as_mut_bytes(&mut data));
        data[0] as f32 / 1000.0
    }

    /// Return the latest Pressure sensor values.
    pub fn read_pressure(&self) -> Pressure {
        // create read buffer
        let mut data = [0i32; get_buffer_length(PRESSURE_BYTES)];

        let address = fpga_address::MCU + (mcu_offset::PRESSURE >> 1);
        // populate buffer
        self.bus.read(address, as_mut_bytes(&mut data));
        Pressure {
            pressure: data[1] as f32 / 1000.0,
            altitude: data[0] as f32 / 1000.0,
            temperature: data[2] as f32 / 1000.0,
        }
    }

    /// Return the latest Humidity sensor values.
    pub fn read_humidity(&self) -> Humidity {
        // create read buffer
        let mut data = [0i32; get_buffer_length(HUMIDITY_BYTES)];
        let address = fpga_address::MCU + (mcu_offset::HUMIDITY >> 1);
        // populate buffer
        self.bus.read(address, as_mut_bytes(&mut data));
        Humidity {
            humidity: data[0] as f32 / 1000.0,
            temperature: data[1] as f32 / 1000.0,
        }
    }

    /// Return the latest IMU sensor values.
    pub fn read_imu(&self) -> Imu {
        // create read buffer
        let mut data = [0i32; get_buffer_length(IMU_BYTES)];

        let address = fpga_address::MCU + (mcu_offset::IMU >> 1);
        // populate read buffer
        self.bus.read(address, as_mut_bytes(&mut data));

        Imu {
            accel_x: data[0] as f32 / 1000.0,
            accel_y: data[1] as f32 / 1000.0,
            accel_z: data[2] as f32 / 1000.0,

            gyro_x: data[3] as f32 / 1000.0,
            gyro_y: data[4] as f32 / 1000.0,
            gyro_z: data[5] as f32 / 1000.0,

            mag_x: data[6] as f32 / 1000.0,
            mag_y: data[7] as f32 / 1000.0,
            mag_z: data[9] as f32 / 1000.0,

            // TODO: ask why we have these. They seem to be unused.
            mag_offset_x: data[9] as f32,
            mag_offset_y: data[10] as f32,
            mag_offset_z: data[11] as f32,

            // These values are already floats so we just need to treat them as one.
            yaw: f32::from_bits(data[12] as u32),
            pitch: f32::from_bits(data[13] as u32),
            roll: f32::from_bits(data[14] as u32),
        }
    }
}

/// Calculate the size a read buffer needs to be for a sensor.
///
/// Since all sensor's values are a byte each, we can divide it by 4 to see how many values need to be stored.
const fn get_buffer_length(sensor_bytes: i32) -> usize {
    (sensor_bytes / 4) as usize
}
