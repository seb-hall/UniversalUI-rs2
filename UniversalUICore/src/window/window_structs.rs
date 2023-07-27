#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::libc::c_char;
use crate::libc::c_void;
use std::ffi::CStr;
use crate::raw_window_handle::*;

use crate::base::*;

pub struct uWindowInfo {
    pub title: *const c_char,
    pub size: uSize,
}

pub type uWindowHandle = u64;