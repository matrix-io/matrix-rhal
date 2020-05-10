#![no_std]
#![no_main]

use esp_idf_sys as idf;
use matrix_rhal::{
    bus::MatrixBus,
    microphone::{mic_core::MicCore, MicArray},
};

#[no_mangle]
pub fn app_main() {
    esp_idf_logger::init().unwrap();

    let bus = matrix_rhal::bus::init();

    log::info!("Freq={}", bus.fpga_frequency(),);

    let mut mics = MicArray::new(&bus);
    let _mic_core = MicCore::new(&mics);

    let everloop = matrix_rhal::Everloop::new(&bus);

    const NUMBER_LEDS: usize =
        matrix_rhal::bus::memory_map::device_info::MATRIX_VOICE_LEDS as usize;
    const LOCAL_AVERAGE_NUM: usize = 20;
    let mut local_average = [0u64; LOCAL_AVERAGE_NUM];
    let mut counter = 0;
    loop {
        mics.read().unwrap(); // Reading 8-mics buffer from the FPGA
        let mut instant_e = 0u64;
        for sample in mics.samples_iter(0) {
            // Increase range of sample to deal with overflow when we multiply
            let sample = *sample as i32;
            instant_e += (sample * sample) as u64;
        }

        local_average[counter % LOCAL_AVERAGE_NUM] = instant_e;
        let mut avg_energy = 0u64;
        for data in local_average.iter() {
            avg_energy += data;
        }

        avg_energy /= LOCAL_AVERAGE_NUM as u64;

        let mut image1d = [matrix_rhal::Rgbw::black(); NUMBER_LEDS];
        for led in image1d.iter_mut() {
            led.r = (avg_energy >> 24) as u8;
        }
        everloop.set(&image1d);

        counter += 1;
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("PANIC! {}", info);
    unsafe {
        loop {
            let _ = idf::sleep(1);
        }
    }
}
