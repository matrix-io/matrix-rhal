#![no_std]
#![no_main]

use core::convert::TryFrom;
use esp_idf::wifi::Config;

include!("config.rs");

unsafe extern "C" fn _event_loop_callback(
    ctx: *mut core::ffi::c_void,
    event: *mut esp_idf::sys::system_event_t,
) -> esp_idf::sys::esp_err_t {
    0
}

unsafe extern "C" fn event_handler(
    _event_handler_arg: *mut core::ffi::c_void,
    event_base: esp_idf::sys::esp_event_base_t,
    event_id: i32,
    _event_data: *mut core::ffi::c_void,
) {
    use esp_idf::event::*;
    match Event::try_from((event_base, event_id)) {
        Ok(Event::Wifi(WifiEvent::StaStart)) | Ok(Event::Wifi(WifiEvent::StaDisconnected)) => {
            log::info!("esp_wifi_connect trying...");
            esp_idf::wifi::connect().unwrap();
            log::info!("esp_wifi_connect ok.");
        },
        Ok(Event::Ip(IpEvent::StaGotIp)) => 
            // ip_event_got_ip_t* event = (ip_event_got_ip_t*) event_data;
            // ESP_LOGI(TAG, "got ip: %s", ip4addr_ntoa(&event->ip_info.ip));
            log::info!("GOT_IP!"),
        Err(_) => log::warn!("Unhandled event: {:?} {}", EventBase::try_from(event_base), event_id),
        _ => {},
    }
}

fn wifi() -> Result<(), esp_idf::error::Error> {
    use esp_idf::*;

    //https://github.com/espressif/esp-idf/tree/v4.0/examples/wifi
    nvs::init()?;
    tcpip::init();
    //let retval = idf::esp_event_loop_init(Some(event_loop_callback), core::ptr::null_mut());
    event::loop_create_default()?;
    let cfg = wifi::InitConfig::default();
    wifi::init(cfg)?;
    event::handler_register(event::events::ip::StaGotIp, event_handler)?;
    event::handler_register(event::events::wifi::Any, event_handler)?;

    let config = wifi::StaConfig::from(&CONFIG).unwrap();
    wifi::set_mode(wifi::WifiMode::STA)?;
    wifi::set_sta_config(config)?;
    wifi::start()
}

#[no_mangle]
pub fn app_main() {
    esp_idf_logger::init().unwrap();

    wifi().unwrap();

    log::info!("OK");
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        abort();
    }
}
