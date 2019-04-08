#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod bindings;
pub use bindings as zephyr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Device<T: 'static + DriverApi> {
    pub config: &'static DeviceConfig<T>,
    pub driver_api: &'static T,
    pub driver_data: usize,
}

unsafe impl<T: DriverApi> Sync for Device<T> {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfig<T: 'static + DriverApi> {
    pub name: &'static [u8],
    pub init: unsafe extern "C" fn(device: *mut Device<T>) -> cty::c_int,
    pub config_info: usize,
}

unsafe impl<T: DriverApi> Sync for DeviceConfig<T> {}

pub trait DriverApi {}
impl DriverApi for bindings::uart_driver_api {}