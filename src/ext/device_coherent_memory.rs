#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::*;
use std::ffi::c_void;
use std::ptr;
use std::ptr::null_mut;
// pub const SPEC_VERSION: u32 = 1;
// pub const EXTENSION_NAME: *const c_char = b"VK_AMD_device_coherent_memory\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceCoherentMemory: VkBool32,
}

impl Default for VkPhysicalDeviceCoherentMemoryFeaturesAMD {
    fn default() -> Self {
        Self {
            sType: VkStructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
            pNext: null_mut(),
            deviceCoherentMemory: VkBool32::FALSE,
        }
    }
}
