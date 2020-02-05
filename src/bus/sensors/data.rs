/// Structs representing the values of each sensor.

#[derive(Debug)]
pub struct Pressure {
    pub pressure: f32,
    pub altitude: f32,
    pub temperature: f32,
}

#[derive(Debug)]
pub struct Humidity {
    pub humidity: f32,
    pub temperature: f32,
}

#[derive(Debug)]
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
