#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::ptr;

use crate::*;

bitmasks! {
    VkAndroidSurfaceCreateFlagsKHR = enum VkAndroidSurfaceCreateFlagBitsKHR {
        _RESERVED = 0,
    },
}

pub type ANativeWindow = c_void;
#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkAndroidSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkAndroidSurfaceCreateFlagsKHR,
    pub window: *const ANativeWindow,
}
impl Default for VkAndroidSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkAndroidSurfaceCreateInfoKHR {
            sType: VkStructureType::ANDROID_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            window: ptr::null(),
        }
    }
}