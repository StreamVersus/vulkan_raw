#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::ptr;

use crate::*;

bitmasks! {
    VkXcbSurfaceCreateFlagsKHR = enum VkXcbSurfaceCreateFlagBitsKHR {
        _RESERVED = 0,
    },
    VkXlibSurfaceCreateFlagsKHR = enum VkXlibSurfaceCreateFlagBitsKHR {
        _RESERVED = 0,
    },
    VkWaylandSurfaceCreateFlagsKHR = enum VkWaylandSurfaceCreateFlagBitsKHR {
        _RESERVED = 0,
    }
}

pub type xcb_window_t = u32;
pub type xcb_connection_t = c_void;
pub type Window = usize;
pub type Display = c_void;
pub type wl_display = c_void;
pub type wl_surface = c_void;

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *const xcb_connection_t,
    pub window: xcb_window_t,
}
impl Default for VkXcbSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkXcbSurfaceCreateInfoKHR {
            sType: VkStructureType::XCB_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            connection: ptr::null(),
            window: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXlibSurfaceCreateFlagsKHR,
    pub dpy: *const Display,
    pub window: Window,
}
impl Default for VkXlibSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkXlibSurfaceCreateInfoKHR {
            sType: VkStructureType::XLIB_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            dpy: ptr::null(),
            window: Default::default(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *const wl_display,
    pub surface: *const wl_surface,
}
impl Default for VkWaylandSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkWaylandSurfaceCreateInfoKHR {
            sType: VkStructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            display: ptr::null(),
            surface: ptr::null(),
        }
    }
}