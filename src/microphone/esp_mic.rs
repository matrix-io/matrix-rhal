use crate::*;

const MIC_ARRAY_IRQ: esp_idf_sys::gpio_num_t = 5;

static mut G_IRQ_QUEUE: esp_idf_sys::QueueHandle_t = core::ptr::null_mut();

unsafe extern "C" fn isr_handler(arg1: *mut ::core::ffi::c_void) {
    let gpio = MIC_ARRAY_IRQ;
    let retval = esp_idf::freertos::xQueueSendToBackFromISR(G_IRQ_QUEUE, &gpio as *const esp_idf_sys::gpio_num_t as *const _, core::ptr::null_mut());
    //assert_eq!(retval, true);
}

pub struct Mic();

impl Mic {

    pub fn setup_event() -> Result<(), Error> {
        
        let gpio_config = esp_idf_sys::gpio_config_t {
            pin_bit_mask: esp_idf::driver::GpioSel::Sel5 as u64,
            mode: esp_idf_sys::gpio_mode_t_GPIO_MODE_INPUT,
            pull_up_en: esp_idf_sys::gpio_pullup_t_GPIO_PULLUP_DISABLE,
            pull_down_en: esp_idf_sys::gpio_pulldown_t_GPIO_PULLDOWN_ENABLE,
            intr_type: esp_idf_sys::gpio_int_type_t_GPIO_INTR_ANYEDGE,
        };
        unsafe {
            G_IRQ_QUEUE = esp_idf::freertos::xQueueCreate(10, core::mem::size_of::<esp_idf_sys::gpio_num_t>() as u32);
            if G_IRQ_QUEUE == core::ptr::null_mut() {
                return Err(Error::Failed);
            }
            idf!(esp_idf_sys::gpio_config(&gpio_config))?;
            idf!(esp_idf_sys::gpio_install_isr_service(0))?;
            idf!(esp_idf_sys::gpio_isr_handler_add(MIC_ARRAY_IRQ, Some(isr_handler), core::ptr::null_mut()))?;
            Ok(())
        }
    }

    pub fn wait_event() {
        let mut gpio = MIC_ARRAY_IRQ;
        let retval = unsafe {
            esp_idf::freertos::xQueueReceive(G_IRQ_QUEUE, &mut gpio as *mut esp_idf_sys::gpio_num_t as *mut _, esp_idf::freertos::PORT_MAX_DELAY)
        };
        assert_eq!(retval, true);
    }
}