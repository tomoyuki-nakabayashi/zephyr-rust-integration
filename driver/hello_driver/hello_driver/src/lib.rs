#![no_std]

use zephyr_ffi::{print, println};
use bindings::{Device, DeviceConfig};

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello from Rust!\n");
}

unsafe extern "C" fn my_init(_device: *mut Device) -> cty::c_int {
    println!("Hello from My Driver!\n");
    0
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("panic! {:?}", info);
    loop {}
}

#[link_section = ".init_POST_KERNEL40"]
static __DEVICE_MY_DEVICE: Device = Device {
    config: &__CONFIG_MY_DEVICE,
    driver_api: 0,
    driver_data: 0
};

#[link_section = ".devconfig.init"]
static __CONFIG_MY_DEVICE: DeviceConfig = DeviceConfig {
    name: 0,
    init: my_init,
    config_info: 0
};