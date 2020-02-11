use crate::bus::memory_map::*;
use crate::error::Error;
use crate::Bus;
mod data;
use data::*;

pub struct Sensors<'a> {
    pub bus: &'a Bus,
}

// Reading function for each sensor.
// Each value in a sensor is 4 bytes(f32).
// The length for bus.read is determined by (# of sensor properties)*4.
impl<'a> Sensors<'a> {
    /// Creates a new instance of Sensors
    pub fn new(bus: &Bus) -> Sensors {
        Sensors { bus }
    }

    /// Return the latest UV sensor value.
    pub fn read_uv(&mut self) -> f32 {
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(UV_BYTES);
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

        // populate buffer
        self.bus.read(
            fpga_address::MCU + (mcu_offset::UV >> 1),
            &mut data,
            UV_BYTES,
        );

        // update UV value
        data[2] as f32 / 1000.0
    }

    /// Return the latest Pressure sensor values.
    pub fn read_pressure(&mut self) -> Pressure {
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(PRESSURE_BYTES);
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

        // populate buffer
        self.bus.read(
            fpga_address::MCU + (mcu_offset::PRESSURE >> 1),
            &mut data,
            PRESSURE_BYTES,
        );

        Pressure {
            pressure: data[3] as f32 / 1000.0,
            altitude: data[2] as f32 / 1000.0,
            temperature: data[4] as f32 / 1000.0,
        }
    }

    /// Return the latest Humidity sensor values.
    pub fn read_humidity(&mut self) -> Humidity {
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(HUMIDITY_BYTES);
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

        // populate buffer
        self.bus.read(
            fpga_address::MCU + (mcu_offset::HUMIDITY >> 1),
            &mut data,
            HUMIDITY_BYTES,
        );

        Humidity {
            humidity: data[2] as f32 / 1000.0,
            temperature: data[3] as f32 / 1000.0,
        }
    }

    /// Return the latest IMU sensor values.
    pub fn read_imu(&mut self) -> Imu {
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(IMU_BYTES);
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

        // populate read buffer
        self.bus.read(
            fpga_address::MCU + (mcu_offset::IMU >> 1),
            &mut data,
            IMU_BYTES,
        );

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

            mag_offset_x: data[11] as f32 / 1000.0,
            mag_offset_y: data[12] as f32 / 1000.0,
            mag_offset_z: data[13] as f32 / 1000.0,

            yaw: data[14] as f32 / 1000.0,
            pitch: data[15] as f32 / 1000.0,
            roll: data[16] as f32 / 1000.0,
        }
    }
}

/// Calculate the size a read buffer needs to be for a sensor.
/// Since all sensor's values are a byte each, we can divide it by 4 to see how many values we have to store.
/// 2 is added to make room for the FPGA address and the number of bytes to allocate in the IOCTL read call.
const fn get_buffer_length(sensor_bytes: i32) -> usize {
    (sensor_bytes / 4 + 2) as usize
}
