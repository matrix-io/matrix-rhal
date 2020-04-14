use super::*;
use crate::{bus::memory_map::*, error::Error, Device};
use core::convert::TryFrom;

#[derive(Debug)]
pub struct Bus {
    spi: esp_idf_sys::spi_device_handle_t,
    fpga_frequency: u32,
}

struct GpioPin {
    pub index: i32,
}

const FPGA_SPI_CS: GpioPin = GpioPin { index: 23 };
const FPGA_SPI_MOSI: GpioPin = GpioPin { index: 33 };
const FPGA_SPI_MISO: GpioPin = GpioPin { index: 21 };
const FPGA_SPI_SCLK: GpioPin = GpioPin { index: 32 };
const BUFFER_SIZE: usize = 512;

impl Bus {
    pub fn init() -> Result<Bus, Error> {
        // Based on:
        // https://github.com/matrix-io/matrixio_hal_esp32/blob/320c897c0790fc7a0c83201f4f05a11a6c453f25/components/hal/wishbone_bus.cpp#L36
        unsafe {
            let bus_config = esp_idf_sys::spi_bus_config_t {
                miso_io_num: FPGA_SPI_MISO.index,
                mosi_io_num: FPGA_SPI_MOSI.index,
                sclk_io_num: FPGA_SPI_SCLK.index,
                quadwp_io_num: -1,
                quadhd_io_num: -1,
                // memset(0) the rest
                ..::core::mem::zeroed()
            };
            let device_config = esp_idf_sys::spi_device_interface_config_t {
                mode: 3,
                duty_cycle_pos: 128,
                clock_speed_hz: 8 * 1000 * 1000,
                spics_io_num: FPGA_SPI_CS.index,
                queue_size: 1,
                // memset(0) the rest
                ..::core::mem::zeroed()
            };
            let retval = esp_idf_sys::spi_bus_initialize(
                esp_idf_sys::spi_host_device_t_HSPI_HOST,
                &bus_config,
                1,
            );
            esp_int_into_result(retval)?;
            let mut spi: esp_idf_sys::spi_device_handle_t = core::ptr::null_mut();
            let retval = esp_idf_sys::spi_bus_add_device(
                esp_idf_sys::spi_host_device_t_HSPI_HOST,
                &device_config,
                &mut spi,
            );
            esp_int_into_result(retval)?;
            let mut bus = Bus { spi, fpga_frequency: 0 };
            bus.fpga_frequency = bus.get_fpga_frequency()?;
            Ok(bus)
        }
    }
}

impl Bus {
    fn transfer(
        &self,
        send_buffer: &[u8],
        receive_buffer: &mut [u8],
        size: usize,
    ) -> Result<(), Error> {
        unsafe {
            // Based on:
            // https://github.com/matrix-io/matrixio_hal_esp32/blob/320c897c0790fc7a0c83201f4f05a11a6c453f25/components/hal/wishbone_bus.cpp#L77
            let mut transaction = esp_idf_sys::spi_transaction_t {
                length: 8 * size,
                rxlength: 8 * size,
                __bindgen_anon_1: esp_idf_sys::spi_transaction_t__bindgen_ty_1 {
                    tx_buffer: send_buffer.as_ptr() as *const _,
                },
                __bindgen_anon_2: esp_idf_sys::spi_transaction_t__bindgen_ty_2 {
                    rx_buffer: receive_buffer.as_mut_ptr() as *mut _,
                },
                ..core::mem::zeroed()
            };
            let retval = esp_idf_sys::spi_device_transmit(self.spi, &mut transaction);
            esp_int_into_result(retval)
        }
    }
    
    /// Use SPI request to make uncached read of FPGA frequency
    fn get_fpga_frequency(&self) -> Result<u32, Error> {
        // Based off:
        // https://github.com/matrix-io/matrixio_hal_esp32/blob/320c897c0790fc7a0c83201f4f05a11a6c453f25/components/hal/wishbone_bus.cpp#L132
        // The original C:
        // ```c
        // esp_err_t WishboneBus::GetFPGAFrequency() {
        // uint16_t values[2];
        // esp_err_t ret =
        //     SpiRead(kConfBaseAddress + 4, (unsigned char *)values, sizeof(values));
        // fpga_frequency_ = (kFPGAClock * values[1]) / values[0];
        // return ret;
        // }
        // ```
        union FpgaData {
            halfwords: [u16; 2],
            bytes: [u8; 4],
        }
        let mut data = FpgaData { bytes: [0u8; 4] };
        self.read_address(fpga_address::CONF + 4, unsafe { &mut data.bytes });

        let frequency = unsafe {
            let value0 = data.halfwords[0] as u32;
            let value1 = data.halfwords[1] as u32;
            (device_info::FPGA_CLOCK * value1) / value0
        };
        Ok(frequency)
    }

    /// Use SPI to read from `address`, `read_buffer.len()` bytes into `read_buffer`.
    fn read_address(&self, address: u16, read_buffer: &mut [u8]) {
        let tx_buffer = {
            let tx_header = spi_address_bytes(address, true);
            let mut tx_buffer = [0u8; BUFFER_SIZE];
            tx_buffer[0..HARDWARE_ADDRESS_BYTES].copy_from_slice(&tx_header);
            tx_buffer
        };
        let mut rx_buffer = [0u8; BUFFER_SIZE];
        self.transfer(&tx_buffer, &mut rx_buffer, read_buffer.len() + HARDWARE_ADDRESS_BYTES).unwrap();
        for (dst, src) in read_buffer.iter_mut().zip(rx_buffer.iter().skip(HARDWARE_ADDRESS_BYTES)) {
            *dst = *src;
        }
    }

    /// Use SPI to write to `address`, `write_buffer.len()` bytes from `write_buffer`.
    fn write_address(&self, address: u16, write_buffer: &[u8]) {
        let tx_buffer = {
            let tx_header = spi_address_bytes(address, false);
            let mut tx_buffer = [0u8; BUFFER_SIZE];
            tx_buffer[0..HARDWARE_ADDRESS_BYTES].copy_from_slice(&tx_header);
            for (dst, src) in tx_buffer.iter_mut().skip(HARDWARE_ADDRESS_BYTES).zip(write_buffer.iter()) {
                *dst = *src;
            }
            tx_buffer
        };
        let mut rx_buffer = [0u8; BUFFER_SIZE];
        self.transfer(&tx_buffer, &mut rx_buffer, write_buffer.len() + HARDWARE_ADDRESS_BYTES).unwrap();
    }
}

/// Command placed in SPI transmit buffer from the original C version:
/// ```c
/// struct hardware_address {
///     uint8_t readnwrite: 1;
///     uint16_t reg: 15;
/// }
/// ```
type HardwareAddress = [u8; core::mem::size_of::<u16>()];
/// Size of `HardwareAddress` in bytes
const HARDWARE_ADDRESS_BYTES: usize = core::mem::size_of::<HardwareAddress>();

/// Construct command placed in SPI transmit buffer.
/// In the original C version:
/// ```c
/// struct hardware_address {
///     uint8_t readnwrite: 1;
///     uint16_t reg: 15;
/// }
/// ...
/// hardware_address *hw_addr = reinterpret_cast<hardware_address *>(bytes);
/// hw_addr->reg = add;
/// hw_addr->readnwrite = 1;
/// ```
/// 
/// Sample values:
/// reg,readnwrite | bytes [0] [1] |
/// |-|-|
/// | 4,1 | 0x0900 | b00001001 0..0
/// | 8,1 | 0x1100 | b00010001 0..0
/// | 256,1 | 0x0102 | b0..1 0..10
/// 
/// It's clear the values are treated as a `u16`:
/// | 15-1 | 0 |
/// | reg | readnwrite |
/// And then, because the ESP32 is litte-endian stored as __bits 7-0__, __bits 15-8__
fn spi_address_bytes(address: u16, readnwrite: bool) -> HardwareAddress {
    let readnwrite = if readnwrite { 1 } else { 0 };
    ((address << 1) | readnwrite).to_ne_bytes()
}

impl MatrixBus for Bus {
    fn write(&self, write_buffer: &mut [u8]) {
        // Unpack the write address from the first 32-bits
        let buffer_u32 = unsafe { core::intrinsics::transmute::<&mut [u8], &mut [u32]>(write_buffer) };
        let address = u16::try_from(buffer_u32[0]).unwrap();
        // Write actual data to the address
        self.write_address(address, &write_buffer[crate::MATRIXBUS_HEADER_BYTES..])
    }

    fn read(&self, read_buffer: &mut [u8]) {
        // Unpack the read address from the first 32-bits
        let buffer_u32 = unsafe { core::intrinsics::transmute::<&mut [u8], &mut [u32]>(read_buffer) };
        let address = u16::try_from(buffer_u32[0]).unwrap();
        self.read_address(address, &mut read_buffer[crate::MATRIXBUS_HEADER_BYTES..])
    }

    fn close(&self) {
        // Do nothing
    }

    fn device_name(&self) -> Device {
        // Currently Matrix Voice is only device with ESP32
        Device::Voice
    }

    fn device_version(&self) -> u32 {
        unimplemented!()
    }

    fn device_leds(&self) -> u8 {
        // Currently Matrix Voice is only option with ESP32
        device_info::MATRIX_VOICE_LEDS
    }

    fn fpga_frequency(&self) -> u32 {
        self.fpga_frequency
    }
}
