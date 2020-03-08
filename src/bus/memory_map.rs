/// FPGA Wishbone address map.
pub mod fpga_address {
    pub const CONF: u16 = 0x0000;
    pub const UART: u16 = 0x1000;
    pub const MICROPHONE_ARRAY: u16 = 0x2000;
    pub const EVERLOOP: u16 = 0x3000;
    pub const GPIO: u16 = 0x4000;
    pub const MCU: u16 = 0x5000;
    pub const AUDIO_OUTPUT: u16 = 0x6000;
    pub const ZWAVE_GPIO: u16 = 0x7000;
}

/// MCU memory address offsets.
pub mod mcu_offset {
    pub const UV: u16 = 0x00;
    pub const PRESSURE: u16 = 0x04;
    pub const HUMIDITY: u16 = 0x10;
    pub const IMU: u16 = 0x30;
    pub const MCU: u16 = 0x90;
}

/// General MATRIX device information.
pub mod device_info {
    pub const MATRIX_CREATOR_LEDS: u8 = 35;
    pub const MATRIX_VOICE_LEDS: u8 = 18;
    pub const MATRIX_CREATOR: i32 = 0x05C344E8;
    pub const MATRIX_VOICE: i32 = 0x6032BAD2;
    pub const FPGA_CLOCK: u32 = 50000000; // Physical OSC = 50MHz
}

/// Request codes for IOCTL read/write functionality.
pub mod ioctl_code {
    /// IOCTL write request code.
    pub const WRITE: i32 = 1200;
    /// IOCTL read request code.
    pub const READ: i32 = 1201;
}
