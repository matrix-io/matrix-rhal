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
