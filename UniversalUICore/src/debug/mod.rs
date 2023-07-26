//  UniversalUICore crate - debug/mod.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  debug/mod.rs defines functions for debugging.

extern crate libc;
extern crate colored;

use self::libc::c_char;
use std::ffi::CStr;

use debug::colored::Colorize;

//  information only, no issues
#[no_mangle]
pub unsafe extern "C" fn debug_info(message: *const c_char) {
    let message_string: &CStr = unsafe { CStr::from_ptr(message) };
    println!(
        "{} {}",
        "[UUI-INFO]:".cyan(),
        message_string.to_str().unwrap()
    )
}

pub fn internal_debug_info(message: &str) {
    println!(
        "{} {}",
        "[UUI-INFO]:".cyan(),
        message
    )
}

//  warning, no significant issues to functionality but
//  non ideal implementation
#[no_mangle]
pub unsafe extern "C" fn debug_warning(message: *const c_char) {
    let message_string: &CStr = unsafe { CStr::from_ptr(message) };
    println!(
        "{} {}",
        "[UUI-WARNING]:".yellow(),
        message_string.to_str().unwrap()
    )
}

pub fn internal_debug_warning(message: &str) {
    println!(
        "{} {}",
        "[UUI-WARNING]:".yellow(),
        message
    )
}


//  error, wrong implementation and functionality affected.
//  Typically used when using a function incorrectly.
#[no_mangle]
pub unsafe extern "C" fn debug_error(message: *const c_char) {
    let message_string: &CStr = unsafe { CStr::from_ptr(message) };
    println!(
        "{} {}",
        "[UUI-ERROR]:".bright_yellow(),
        message_string.to_str().unwrap()
    )
}

pub fn internal_debug_error(message: &str) {
    println!(
        "{} {}",
        "[UUI-ERROR]:".bright_yellow(),
        message
    )
}

//  critical error, app about to crash
#[no_mangle]
pub unsafe extern "C" fn debug_critical(message: *const c_char) {
    let message_string: &CStr = unsafe { CStr::from_ptr(message) };
    println!(
        "{} {}",
        "[UUI-CRITICAL]:".bright_red(),
        message_string.to_str().unwrap()
    )
}

pub fn internal_debug_critical(message: &str) {
    println!(
        "{} {}",
        "[UUI-CRITICAL]:".bright_red(),
        message
    )
}