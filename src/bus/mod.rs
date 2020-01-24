pub struct Bus {
    DeviceName: String,
    RxBuffer: [u8; 12288],
    TxBuffer: [u8; 12288],
    RegMapFd: i16,
    // TODO: add mutex
}

impl Bus {
    pub fn init(device_name: String) -> bool {
        false
    }

    pub fn write(add: u16, data: &u8, length: i16) -> bool {
        false
    }

    pub fn read(add: u16, data: &mut u8, length: i16) -> bool {
        false
    }
}
