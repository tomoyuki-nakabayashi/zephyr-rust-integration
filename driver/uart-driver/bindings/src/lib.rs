#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod bindings;
pub use bindings as zephyr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Device {
    pub config: &'static DeviceConfig,
    pub driver_api: &'static zephyr::uart_driver_api,
    pub driver_data: usize,
}

unsafe impl Sync for Device {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfig {
    pub name: &'static [u8],
    pub init: unsafe extern "C" fn(device: *mut Device) -> cty::c_int,
    pub config_info: usize,
}

unsafe impl Sync for DeviceConfig {}