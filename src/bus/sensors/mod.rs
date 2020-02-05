use super::kernel::Bus;
use super::memory_map::*;
mod data;
use data::*;

impl<'a> Bus<'a> {
    pub fn read_uv(&mut self) -> f32 {
        // store the bytes representing UV value
        self.read(K_MCU_BASE_ADDRESS + (K_MEMORY_OFFSET_UV >> 1), 4);
        self.rx_buffer[2] as f32 / 1000.0
    }

    pub fn read_pressue(&mut self) -> Pressure {
        // store the bytes representing pressure values
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
}
