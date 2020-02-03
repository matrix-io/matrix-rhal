/* FPGA Wishbone address map */
pub const K_CONF_BASE_ADDRESS: u16 = 0x0000;
pub const K_UART_BASE_ADDRESS: u16 = 0x1000;
pub const K_MICROPHONE_ARRAY_BASE_ADDRESS: u16 = 0x2000;
pub const K_EVERLOOP_BASE_ADDRESS: u16 = 0x3000;
pub const K_GPIO_BASE_ADDRESS: u16 = 0x4000;
pub const K_MCU_BASE_ADDRESS: u16 = 0x5000;
pub const K_AUDIO_OUTPUT_BASE_ADDRESS: u16 = 0x6000;
pub const K_ZWAVE_GPIO_BASE_ADDRESS: u16 = 0x7000;

/* MCU offsets map */
pub const K_MEMORY_OFFSET_UV: u16 = 0x00;
pub const K_MEMORY_OFFSET_PRESSURE: u16 = 0x04;
pub const K_MEMORY_OFFSET_HUMIDITY: u16 = 0x10;
pub const K_MEMORY_OFFSET_IMU: u16 = 0x30;
pub const K_MEMORY_OFFSET_MCU: u16 = 0x90;

/* MATRIX device variables */
pub const K_MATRIX_CREATOR_N_LEDS: i32 = 35;
pub const K_MATRIX_VOICE_N_LEDS: i32 = 18;
pub const K_MATRIX_CREATOR: i32 = 0x05C344E8;
pub const K_MATRIX_VOICE: i32 = 0x6032BAD2;
pub const kFPGAClock: u32 = 50000000; // Physical OSC = 50MHz
