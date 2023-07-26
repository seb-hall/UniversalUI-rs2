#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::super::libc::c_char;
use super::super::libc::c_void;
use std::ffi::CStr;
use super::super::raw_window_handle::*;

use crate::base::*;

pub struct uWindowInfo {
    pub title: *const c_char,
    pub size: uSize,
}

pub struct uWindowHandle {
    pub raw_handle: *mut c_void,
}