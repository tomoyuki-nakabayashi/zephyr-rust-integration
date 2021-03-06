#![no_std]
#![allow(dead_code)]

use bindings::zephyr::{self, device, uart_driver_api};
use bindings::{Device, DeviceConfig};
use zephyr_ffi::{print, println};

// Flag register
const FR: u32 = 0x18;
const UARTFR_RXFE: u32 = 0x00000010;

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello from Rust!\n");
}

unsafe extern "C" fn my_init(_device: *mut Device<uart_driver_api>) -> cty::c_int {
    0
}

unsafe extern "C" fn rust_poll_out(_dev: *mut device, out_char: cty::c_uchar) {
    *(zephyr::UART_0_BASE_ADDRESS as *mut u32) = out_char as u32;
}

unsafe extern "C" fn rust_poll_in(_dev: *mut device, p_char: *mut cty::c_uchar) -> cty::c_int {
    let flags = *((zephyr::UART_0_BASE_ADDRESS + FR) as *const u32);
    if (flags & UARTFR_RXFE) != 0 {
        return -1; // don't have RX data.
    }

    *p_char = *(zephyr::UART_0_BASE_ADDRESS as *mut u32) as cty::c_uchar;
    return 0;
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("panic! {:?}", info);
    loop {}
}

static UART_API: uart_driver_api = uart_driver_api {
    poll_out: Some(rust_poll_out),
    poll_in: Some(rust_poll_in),
    err_check: None,
    configure: None,
    config_get: None,
};

#[macro_export]
macro_rules! device_config {
    ($dev_name:ident, $name:expr, $init:expr, $info:expr) => {
        #[link_section = ".devconfig.init"]
        static $dev_name: DeviceConfig<uart_driver_api> = DeviceConfig::<uart_driver_api> {
            name: $name,
            init: $init,
            config_info: $info,
        };
    };
}

#[macro_export]
macro_rules! device_init {
    ($dev_name:ident, $config:expr, $api:expr, $data:expr) => {
        #[link_section = ".init_POST_KERNEL40"]
        static $dev_name: Device<uart_driver_api> = Device::<uart_driver_api> {
            config: $config,
            driver_api: $api,
            driver_data: $data,
        };
    };
}

device_config!(
    __CONFIG_MY_DEVICE,
    zephyr::CONFIG_UART_CONSOLE_ON_DEV_NAME,
    my_init,
    0
);
device_init!(__DEVICE_MY_DEVICE, &__CONFIG_MY_DEVICE, &UART_API, 0);
