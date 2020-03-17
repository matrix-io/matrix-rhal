#![no_std]
#![no_main]

extern crate matrix_rhal;

use esp_idf_sys as esp;

fn is_little_endian() -> bool {
    let value = 1u32;
    let value = unsafe { *(&value as *const u32 as *const u8) };
    value == 1
}

#[no_mangle]
pub fn app_main() {
    unsafe {
        esp::ets_printf(
            b"Little-endian=%d\n\0".as_ptr() as *const _,
            is_little_endian() as u32,
        );
        let bus = matrix_rhal::bus::init();
        let everloop = matrix_rhal::Everloop::new(&bus);

        let mut counter = 0;
        loop {
            const NUMBER_LEDS: usize =
                matrix_rhal::bus::memory_map::device_info::MATRIX_VOICE_LEDS as usize;
            let mut image1d = [matrix_rhal::Rgbw::black(); NUMBER_LEDS];
            image1d[(counter / 2) % NUMBER_LEDS].r = 20;
            image1d[(counter / 7) % NUMBER_LEDS].g = 30;
            image1d[(counter / 11) % NUMBER_LEDS].b = 30;
            image1d[NUMBER_LEDS - 1 - (counter % NUMBER_LEDS)].w = 10;
            everloop.set(&image1d);

            counter += 1;
            
            // Set RTC timer to trigger wakeup and then enter light sleep
            esp::esp_sleep_enable_timer_wakeup(25000);
            esp::esp_light_sleep_start();
        }
    }
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        //ets_printf(b"panic!\n\0".as_ptr() as *const _);
        abort();
    }
}
