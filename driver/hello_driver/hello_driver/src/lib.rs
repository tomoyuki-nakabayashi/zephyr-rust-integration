#![no_std]

use zephyr_ffi::{print, println};
use bindings::{Device, DeviceConfig};
use bindings::zephyr::{self, device, uart_driver_api};

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello from Rust!\n");
}

unsafe extern "C" fn my_init(_device: *mut Device) -> cty::c_int {
    0
}

unsafe extern "C" fn rust_poll_out(_dev: *mut device, out_char: cty::c_uchar)
{
    *(zephyr::UART_0_BASE_ADDRESS as *mut u32) = out_char as u32;
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
    poll_in: None,
    err_check: None,
    configure: None,
    config_get: None,
};

#[link_section = ".init_POST_KERNEL40"]
static __DEVICE_MY_DEVICE: Device = Device {
    config: &__CONFIG_MY_DEVICE,
    driver_api: &UART_API,
    driver_data: 0
};

#[link_section = ".devconfig.init"]
static __CONFIG_MY_DEVICE: DeviceConfig = DeviceConfig {
    name: zephyr::CONFIG_UART_CONSOLE_ON_DEV_NAME,
    init: my_init,
    config_info: 0
};