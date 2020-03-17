use super::MatrixBus;

/// Uses the SPI through the Raspberry Pi.
pub mod direct;
/// Uses the MATRIX Kernel Modules.
pub mod kernel;

/// Return a Bus type that communicates with the MATRIX Bus on the FPGA.
pub fn init() -> Box<dyn MatrixBus> {
    // use bus through MATRIX Kernel Modules
    if let Ok(bus) = kernel::Bus::init() {
        return Box::new(bus);
    }
    // or through Raspberry Pi SPI
    else {
        return Box::new(direct::Bus::init().expect("Could not communicate with the MATRIX Bus through the Kernel Modules or the Raspberry PI's SPI"));
    }
}
