use crate::bus::memory_map::*;
use crate::Bus;
mod data;
use data::*;

/// Communicates with the main sensors on the MATRIX Creator.
pub struct Sensors<'a> {
    pub bus: &'a Bus,
}

// Read function for each sensor.
// Each value in a sensor is 4 bytes(f32).
// The length for bus.read is determined by (# of sensor properties)*4.
impl<'a> Sensors<'a> {
    /// Creates a new instance of Sensors.
    pub fn new(bus: &Bus) -> Sensors {
        Sensors { bus }
    }

    /// Return the latest UV sensor value.
    pub fn read_uv(&self) -> f32 {
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(UV_BYTES);
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

        data[0] = (fpga_address::MCU + (mcu_offset::UV >> 1)) as i32;
        data[1] = UV_BYTES;

        // populate buffer
        self.bus
            .read(unsafe { std::mem::transmute::<&mut [i32], &mut [u8]>(&mut data) });

        // update UV value
        data[2] as f32 / 1000.0
    }

    /// Return the latest Pressure sensor values.
    pub fn read_pressure(&self) -> Pressure {
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(PRESSURE_BYTES);
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
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(HUMIDITY_BYTES);
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
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(IMU_BYTES);
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
            yaw: unsafe { std::mem::transmute::<i32, f32>(data[14]) },
            pitch: unsafe { std::mem::transmute::<i32, f32>(data[15]) },
            roll: unsafe { std::mem::transmute::<i32, f32>(data[16]) },
        }
    }
}

/// Calculate the size a read buffer needs to be for a sensor.
/// Since all sensor's values are a byte each, we can divide it by 4 to see how many values we have to store.
/// 2 is added to make room for the FPGA address and the number of bytes to allocate in the IOCTL read call.
const fn get_buffer_length(sensor_bytes: i32) -> usize {
    (sensor_bytes / 4 + 2) as usize
}
