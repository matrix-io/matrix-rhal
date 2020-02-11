use crate::bus::memory_map::*;
use crate::error::Error;
use crate::Bus;
mod data;
use data::*;

pub struct Sensors<'a> {
    pub uv: Uv,
    pub imu: Imu,
    pub humidity: Humidity,
    pub pressure: Pressure,
    pub bus: &'a Bus,
}

impl<'a> Sensors<'a> {
    pub fn new(bus: &Bus) -> Sensors {
        Sensors {
            uv: Uv::default(),
            imu: Imu::default(),
            humidity: Humidity::default(),
            pressure: Pressure::default(),
            bus,
        }
    }
}

// Reading function for each sensor.
// Each value in a sensor is 4 bytes(f32).
// The length for bus.read is determined by (# of sensor properties)*4.
impl<'a> Sensors<'a> {
    /// Updates the Uv struct in Sensors with new values.
    pub fn read_uv(&mut self) {
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
        self.uv.uv = data[2] as f32 / 1000.0;
    }

    /// Updates the Pressure struct in Sensors with new values.
    pub fn read_pressure(&mut self) {
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(PRESSURE_BYTES);
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

        // populate buffer
        self.bus.read(
            fpga_address::MCU + (mcu_offset::PRESSURE >> 1),
            &mut data,
            PRESSURE_BYTES,
        );

        // update Pressure values
        self.pressure.pressure = data[3] as f32 / 1000.0;
        self.pressure.altitude = data[2] as f32 / 1000.0;
        self.pressure.temperature = data[4] as f32 / 1000.0;
    }

    /// Updates the Humidity struct in Sensors with new values.
    pub fn read_humidity(&mut self) {
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(HUMIDITY_BYTES);
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

        // populate buffer
        self.bus.read(
            fpga_address::MCU + (mcu_offset::HUMIDITY >> 1),
            &mut data,
            HUMIDITY_BYTES,
        );

        // update humidity values
        self.humidity.humidity = data[2] as f32 / 1000.0;
        self.humidity.temperature = data[3] as f32 / 1000.0;
    }

    /// Updates the Imu struct in Sensors with new values.
    pub fn read_imu(&mut self) {
        // create read buffer
        const BUFFER_LENGTH: usize = get_buffer_length(IMU_BYTES);
        let mut data: [i32; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

        // populate read buffer
        self.bus.read(
            fpga_address::MCU + (mcu_offset::IMU >> 1),
            &mut data,
            IMU_BYTES,
        );

        // update IMU value
        self.imu.accel_x = data[2] as f32 / 1000.0;
        self.imu.accel_y = data[3] as f32 / 1000.0;
        self.imu.accel_z = data[4] as f32 / 1000.0;

        self.imu.gyro_x = data[5] as f32 / 1000.0;
        self.imu.gyro_y = data[6] as f32 / 1000.0;
        self.imu.gyro_z = data[7] as f32 / 1000.0;

        self.imu.mag_x = data[8] as f32 / 1000.0;
        self.imu.mag_y = data[9] as f32 / 1000.0;
        self.imu.mag_z = data[10] as f32 / 1000.0;

        self.imu.mag_offset_x = data[11] as f32 / 1000.0;
        self.imu.mag_offset_y = data[12] as f32 / 1000.0;
        self.imu.mag_offset_z = data[13] as f32 / 1000.0;

        self.imu.yaw = data[14] as f32 / 1000.0;
        self.imu.pitch = data[15] as f32 / 1000.0;
        self.imu.roll = data[16] as f32 / 1000.0;
    }
}

/// Calculate the size a read buffer needs to be for a sensor.
/// Since all sensor's values are a byte each, we can divide it by 4 to see how many values we have to store.
/// 2 is added to make room for the FPGA address and the number of bytes to allocate in the IOCTL read call.
const fn get_buffer_length(sensor_bytes: i32) -> usize {
    (sensor_bytes / 4 + 2) as usize
}
