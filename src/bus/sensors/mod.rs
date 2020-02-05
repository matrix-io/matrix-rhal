use super::kernel::Bus;
use super::memory_map::*;
mod data;
use data::*;

// Reading function for each sensor.
// Each value in a sensor is 4 bytes(f32).
// The length for bus.read is determined by (# of sensor properties)*4.
impl<'a> Bus<'a> {
    pub fn read_uv(&mut self) -> f32 {
        self.read(K_MCU_BASE_ADDRESS + (K_MEMORY_OFFSET_UV >> 1), 4);
        self.rx_buffer[2] as f32 / 1000.0
    }

    pub fn read_pressue(&mut self) -> Pressure {
        self.read(K_MCU_BASE_ADDRESS + (K_MEMORY_OFFSET_PRESSURE >> 1), 12);

        Pressure {
            pressure: self.rx_buffer[3] as f32 / 1000.0,
            altitude: self.rx_buffer[2] as f32 / 1000.0,
            temperature: self.rx_buffer[4] as f32 / 1000.0,
        }
    }

    pub fn read_humidity(&mut self) -> Humidity {
        // store the bytes representing humidity values
        self.read(K_MCU_BASE_ADDRESS + (K_MEMORY_OFFSET_HUMIDITY >> 1), 8);

        Humidity {
            humidity: self.rx_buffer[2] as f32 / 1000.0,
            temperature: self.rx_buffer[3] as f32 / 1000.0,
        }
    }

    pub fn read_imu(&mut self) -> Imu {
        self.read(K_MCU_BASE_ADDRESS + (K_MEMORY_OFFSET_IMU >> 1), 60);

        // TODO: test if values are properly assigned
        Imu {
            accel_x: self.rx_buffer[2] as f32 / 1000.0,
            accel_y: self.rx_buffer[3] as f32 / 1000.0,
            accel_z: self.rx_buffer[4] as f32 / 1000.0,

            gyro_x: self.rx_buffer[5] as f32 / 1000.0,
            gyro_y: self.rx_buffer[6] as f32 / 1000.0,
            gyro_z: self.rx_buffer[7] as f32 / 1000.0,

            mag_x: self.rx_buffer[8] as f32 / 1000.0,
            mag_y: self.rx_buffer[9] as f32 / 1000.0,
            mag_z: self.rx_buffer[10] as f32 / 1000.0,

            mag_offset_x: self.rx_buffer[11] as f32 / 1000.0,
            mag_offset_y: self.rx_buffer[12] as f32 / 1000.0,
            mag_offset_z: self.rx_buffer[13] as f32 / 1000.0,

            yaw: self.rx_buffer[13] as f32 / 1000.0,
            pitch: self.rx_buffer[14] as f32 / 1000.0,
            roll: self.rx_buffer[15] as f32 / 1000.0,
        }
    }
}
