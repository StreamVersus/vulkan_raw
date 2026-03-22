#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::*;
use std::ffi::c_void;
// use std::os::raw::c_char;
use std::ptr;

// pub const SPEC_VERSION: u32 = 1;
// pub const EXTENSION_NAME: *const c_char = b"VK_EXT_memory_priority\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryPriority: VkBool32,
}
impl Default for VkPhysicalDeviceMemoryPriorityFeaturesEXT {
    fn default() -> Self {
        Self {
            sType: VkStructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT,
            pNext: ptr::null_mut(),
            memoryPriority: Default::default(),
        }
    }
}
