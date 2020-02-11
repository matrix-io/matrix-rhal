/// Number of Bytes needed to represent UV data.
pub const UV_BYTES: i32 = 4;
#[derive(Debug, Default)]
pub struct Uv {
    pub uv: f32,
}

/// Number of Bytes needed to represent Pressure data.
pub const PRESSURE_BYTES: i32 = 12;
#[derive(Debug, Default)]
pub struct Pressure {
    pub pressure: f32,
    pub altitude: f32,
    pub temperature: f32,
}

/// Number of Bytes needed to represent Humidity data.
pub const HUMIDITY_BYTES: i32 = 8;
#[derive(Debug, Default)]
pub struct Humidity {
    pub humidity: f32,
    pub temperature: f32,
}

/// Number of Bytes needed to represent IMU data.
pub const IMU_BYTES: i32 = 60;
#[derive(Debug, Default)]
pub struct Imu {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,

    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,

    pub mag_x: f32,
    pub mag_y: f32,
    pub mag_z: f32,

    pub mag_offset_x: f32,
    pub mag_offset_y: f32,
    pub mag_offset_z: f32,

    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
