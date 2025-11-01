#![allow(non_snake_case)]
#![allow(hidden_glob_reexports)]

use std::fmt::Display;
use std::ffi::{c_void, CStr};
use std::fmt::{Debug, Error, Formatter};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

pub type VkSampleMask = u32;
pub type VkFlags = u32;
pub type VkFlags64 = u64;
pub type VkDeviceSize = u64;
pub type VkDeviceAddress = u64;

#[repr(C)]
pub struct VkBaseOutStructure {
    pub sType: VkStructureType,
    pub pNext: *mut VkBaseOutStructure,
}

#[repr(C)]
pub struct VkBaseInStructure {
    pub sType: VkStructureType,
    pub pNext: *const VkBaseInStructure,
}

pub type HANDLE = usize;
pub type HINSTANCE = usize;
pub type HWND = usize;
pub type LPCWSTR = *const u16;
pub type DWORD = u32;
#[allow(non_camel_case_types)]
pub type SECURITY_ATTRIBUTES = c_void; // TODO

#[repr(u32)]
pub enum VkVersion {
    V1_0 = 0x00400000,
    V1_1 = 0x00401000,
    V1_2 = 0x00402000,
    V1_3 = 0x00403000,
    V1_4 = 0x00404000,
}

macro_rules! handle {
    ($name:ident, $type:ident$(,$type_enum_variant:path)?) => {
        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct $name($type);

        impl $name {
            $(
            pub const TYPE: VkObjectType = $type_enum_variant;
            )?

            pub fn none() -> Self {
                $name(<$type>::none())
            }
            
            pub fn handle(&self) -> $type {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name(<$type>::none())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                write!(f, "{}", self.0.to_string())
            }
        }

        impl From<$name> for u64 {
            fn from(handle: $name) -> Self {
                handle.0.into()
            }
        }
        
        impl From<u64> for $name {
            fn from(handle: u64) -> Self {
                $name($type(handle))
            }
        }
        
        impl Handle for $name {
            fn addr(&self) -> u64 {
                self.0.addr()
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
    };
}

#[macro_export]
macro_rules! link_vulkan_structures {
    {
        $($first:ident).+
        =>
        $(#[$last_attr:meta])*
        $($last:ident).+
    } => {
        {
            $(#[$last_attr])*
            {
                $($first).+.pNext = std::mem::transmute(&$($last).+);
            }
        }
    };
    {
        $($first:ident).+
        =>
        $(
            $(#[$other_attr:meta])*
            $($other:ident).+
        ),+
        =>
        $(#[$last_attr:meta])*
        $($last:ident).+
    }=>{
        {
            let mut last_next: &mut *mut std::ffi::c_void = std::mem::transmute(&mut $($first).+.pNext);
            $(
                $(#[$other_attr])*
                {
                    *last_next = std::mem::transmute(&$($other).+);
                    last_next = std::mem::transmute(&mut $($other).+.pNext);
                }
            )+
            $(#[$last_attr])*
            {
                *last_next = std::mem::transmute(&$($last).+);
            }
        }
    }
}

macro_rules! enums {
    (
        $(
            $(#[$enum_attr:meta])*
            enum $enum_name:ident{
                $(
                    $(#[$variant_attr:meta])*
                    $variant_name:ident = $value:literal
                ),*$(,)?
            }
        ),*$(,)?
    )=>{
        $(
            $(#[$enum_attr])*
            #[repr(C)]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
            pub enum $enum_name{
                $(
                    $(#[$variant_attr])*
                    $variant_name = $value,
                )*
            }
        )*
    }
}

macro_rules! bitmasks {
    (
        $(
            $(#[$flags_attr:meta])*
            $flags_name:ident = enum $flag_bits_name:ident{
                $(
                    $(#[$inner:ident $($args:tt)*])*
                    $bit_name:ident = $value:literal
                ),*$(,)?
            }
        ),*$(,)?
    )=>{
        $(
            bitflags! {
                $(#[$flags_attr])*
                #[repr(transparent)]
                #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
                pub struct $flag_bits_name: VkFlags {
                    $(
                        $(#[$inner $($args)*])*
                        const $bit_name = $value;
                    )*
                }
            }
            pub type $flags_name = $flag_bits_name;
        )*
    };
}

#[cfg(feature = "VK_VERSION_1_3")]
macro_rules! bitmasks64 {
    (
        $(
            $(#[$flags_attr:meta])*
            $flags_name:ident = enum $flag_bits_name:ident{
                $(
                    $(#[$inner:ident $($args:tt)*])*
                    $bit_name:ident = $value:literal
                ),*$(,)?
            }
        ),*$(,)?
    )=>{
        $(
            bitflags! {
                $(#[$flags_attr])*
                #[repr(transparent)]
                #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
                pub struct $flag_bits_name: VkFlags64 {
                    $(
                        $(#[$inner $($args)*])*
                        const $bit_name = $value;
                    )*
                }
            }
            pub type $flags_name = $flag_bits_name;
        )*
    };
}

macro_rules! instance_level_functions {
    (
        $(
            $(#[$function_attr:meta])*
            fn $function_name:ident($($parameter_name:ident:$parameter_type:ty),*)$(->$return_type:ty)?;
        )*
    ) => {
        paste::paste! {
            $(
                $(#[$function_attr])*
                static mut [<$function_name _PFN>]: extern "C" fn($($parameter_name:$parameter_type),*)$(->$return_type)? = {
                    extern "C" fn uninitialized_function($(_:$parameter_type),*)$(->$return_type)? {
                        panic!("Vulkan function {} called before initialization!", stringify!($function_name));
                    }
                    uninitialized_function
                };
            )*
        }

        static INSTANCE_FUNCTIONS_INIT: Once = Once::new();

        pub fn load_instance_functions(instance: crate::core::VkInstance) {
            INSTANCE_FUNCTIONS_INIT.call_once(|| {
                use std::ffi::CStr;
                use std::mem::transmute;
                use crate::get_instance_proc_addr;

                paste::paste! {
                    unsafe {
                        $(
                        [<$function_name _PFN>] = match get_instance_proc_addr(
                            instance,
                            CStr::from_bytes_with_nul_unchecked(concat!(stringify!($function_name), '\0').as_bytes()))
                        {
                            Ok(proc_addr) => transmute(proc_addr),
                            Err(_) => {
                                extern "C" fn fallback_function($(_:$parameter_type),*)$(->$return_type)? {
                                    panic!("Vulkan function {} not available", stringify!($function_name));
                                }
                                fallback_function
                            },
                        };
                        )*
                    }
                }
            });
        }

        paste::paste! {
            $(
            $(#[$function_attr])*
            #[inline(always)]
            pub unsafe fn $function_name($($parameter_name:$parameter_type),*)$(->$return_type)? {
                ([<$function_name _PFN>])($($parameter_name),*)
            }
            )*
        }
    }
}

macro_rules! device_level_functions {
    (
        $(
            $(#[$function_attr:meta])*
            fn $function_name:ident($($parameter_name:ident:$parameter_type:ty),*)$(->$return_type:ty)?;
        )*
    ) => {
        paste::paste! {
            $(
                $(#[$function_attr])*
                static mut [<$function_name _PFN>]: extern "C" fn($($parameter_name:$parameter_type),*)$(->$return_type)? = {
                    extern "C" fn uninitialized_function($(_:$parameter_type),*)$(->$return_type)? {
                        panic!("Vulkan function {} called before initialization!", stringify!($function_name));
                    }
                    uninitialized_function
                };
            )*
        }

        static DEVICE_FUNCTIONS_INIT: Once = Once::new();

        pub fn load_device_functions(device: crate::core::VkDevice) {
            DEVICE_FUNCTIONS_INIT.call_once(|| {
                use std::ffi::CStr;
                use std::mem::transmute;
                use crate::get_device_proc_addr;

                paste::paste! {
                    unsafe {
                        $(
                        $(#[$function_attr])*
                        {
                            [<$function_name _PFN>] = match get_device_proc_addr(
                                device,
                                CStr::from_bytes_with_nul_unchecked(concat!(stringify!($function_name), '\0').as_bytes()))
                            {
                                Ok(proc_addr) => transmute(proc_addr),
                                Err(_) => {
                                    extern "C" fn fallback_function($(_:$parameter_type),*)$(->$return_type)? {
                                        panic!("Vulkan function {} not available", stringify!($function_name));
                                    }
                                    fallback_function
                                },
                            };
                        }
                        )*
                    }
                }
            });
        }

        paste::paste! {
            $(
            $(#[$function_attr])*
            #[inline(always)]
            pub unsafe fn $function_name($($parameter_name:$parameter_type),*)$(->$return_type)? {
                ([<$function_name _PFN>])($($parameter_name),*)
            }
            )*
        }
    }
}


#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DispatchableHandle(u64);
impl DispatchableHandle {
    pub fn none() -> Self {
        DispatchableHandle(0)
    }
    
    pub fn addr(&self) -> u64 {
        self.0
    }
}
impl Display for DispatchableHandle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#x}", self.0)
    }
}
impl From<DispatchableHandle> for u64 {
    fn from(handle: DispatchableHandle) -> Self {
        handle.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NonDispatchableHandle(u64);
impl NonDispatchableHandle {
    pub fn none() -> Self {
        NonDispatchableHandle(0)
    }
    
    pub fn addr(&self) -> u64 {
        self.0
    }
}
impl Display for NonDispatchableHandle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}
impl From<NonDispatchableHandle> for u64 {
    fn from(handle: NonDispatchableHandle) -> Self {
        handle.0
    }
}

pub trait Handle {
    fn addr(&self) -> u64;
}

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_LUID_SIZE: usize = 8;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_LOD_CLAMP_NONE: f32 = 1000f32;
pub const VK_REMAINING_MIP_LEVELS: u32 = 0xFFFF_FFFF;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = 0xFFFF_FFFF;
pub const VK_WHOLE_SIZE: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const VK_ATTACHMENT_UNUSED: u32 = 0xFFFF_FFFF;
pub const VK_SHADER_UNUSED_KHR: u32 = 0xFFFF_FFFF;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VkBool32(u32);
impl VkBool32 {
    pub const TRUE: VkBool32 = VkBool32(1);
    pub const FALSE: VkBool32 = VkBool32(0);
}
impl Default for VkBool32 {
    #[inline(always)]
    fn default() -> Self {
        VkBool32::FALSE
    }
}
impl From<VkBool32> for bool {
    #[inline(always)]
    fn from(bool: VkBool32) -> Self {
        bool == VkBool32::TRUE
    }
}
impl From<bool> for VkBool32 {
    #[inline(always)]
    fn from(bool: bool) -> Self {
        VkBool32(bool as u32)
    }
}
impl Debug for VkBool32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            if *self == VkBool32::TRUE {
                "VkBool32::TRUE"
            } else {
                "VkBool32::FALSE"
            }
        )
    }
}
impl Display for VkBool32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", *self == VkBool32::TRUE)
    }
}

pub const VK_QUEUE_FAMILY_IGNORED: u32 = 0xFFFF_FFFF;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = VK_QUEUE_FAMILY_IGNORED - 1;
pub const VK_SUBPASS_EXTERNAL: u32 = 0xFFFF_FFFF;
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;
pub const VK_MAX_DRIVER_NAME_SIZE: usize = 256;
pub const VK_MAX_DRIVER_INFO_SIZE: usize = 256;

#[derive(Debug)]
pub struct LoadingError();

fn get_instance_proc_addr(
    instance: VkInstance,
    name: &CStr,
) -> Result<PFN_vkVoidFunction, LoadingError> {
    let function_pointer = vkGetInstanceProcAddr(instance, name.as_ptr());
    match function_pointer as usize {
        0 => Err(LoadingError()),
        _ => Ok(function_pointer),
    }
}
fn get_device_proc_addr(
    device: VkDevice,
    name: &CStr,
) -> Result<PFN_vkVoidFunction, LoadingError> {
    let function_pointer = unsafe { vkGetDeviceProcAddr(device, name.as_ptr()) };
    match function_pointer as usize {
        0 => Err(LoadingError()),
        _ => Ok(function_pointer),
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ApiVersion(u32);

impl ApiVersion {
    #[inline(always)]
    pub const fn new(major: u32, minor: u32, patch: u32) -> ApiVersion {
        ApiVersion((major << 22) | ((minor & 0x0000_03FF) << 12) | (patch & 0x0000_0FFF))
    }

    #[inline(always)]
    pub fn major(&self) -> u32 {
        (self.0 & 0xFFC0_0000) >> 22
    }

    #[inline(always)]
    pub fn minor(&self) -> u32 {
        (self.0 & 0x003F_F000) >> 12
    }

    #[inline(always)]
    pub fn patch(&self) -> u32 {
        self.0 & 0x0000_0FFF
    }
}

impl From<u32> for ApiVersion {
    #[inline(always)]
    fn from(api_version: u32) -> Self {
        ApiVersion(api_version)
    }
}

impl Into<u32> for ApiVersion {
    #[inline(always)]
    fn into(self) -> u32 {
        self.0
    }
}

impl Display for ApiVersion {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

impl Debug for ApiVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

mod core;
mod ext;
mod khr;

pub use crate::core::*;

#[cfg(feature = "VK_EXT_debug_utils")]
pub use crate::ext::debug_utils::*;

#[cfg(feature = "VK_EXT_index_type_uint8")]
pub use crate::ext::index_type_uint8::*;

#[cfg(feature = "VK_EXT_memory_budget")]
pub use crate::ext::memory_budget::*;

#[cfg(feature = "VK_KHR_external_fence_fd")]
pub use crate::khr::external_fence_fd::*;

#[cfg(feature = "VK_KHR_external_fence_win32")]
pub use crate::khr::external_fence_win32::*;

#[cfg(feature = "VK_KHR_pipeline_library")]
pub use crate::khr::pipeline_library::*;

#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub use crate::khr::deferred_host_operations::*;

#[cfg(feature = "VK_KHR_surface")]
pub use crate::khr::surface::*;

#[cfg(feature = "VK_KHR_swapchain")]
pub use crate::khr::swapchain::*;

#[cfg(feature = "VK_KHR_win32_surface")]
pub use crate::khr::win32_surface::*;

pub use crate::khr::linux_surface::*;

pub use crate::khr::android_surface::*;

#[cfg(feature = "VK_KHR_acceleration_structure")]
pub use crate::khr::acceleration_structure::*;

#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub use crate::khr::ray_tracing_pipeline::*;

#[cfg(feature = "VK_KHR_ray_query")]
pub use crate::khr::ray_query::*;

