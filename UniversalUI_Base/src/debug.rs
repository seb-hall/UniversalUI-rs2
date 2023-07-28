//  UniversalUI_Base - debug.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Base contains definitions for types
//  common to other modules of the UniversalUI framework.
//
//  debug.rs contains functions for printing color-coded
//  messages to the console. These are listed according
//  to severity and allow for a clean printing interface.

use colored::Colorize;

//  information only, no issues.
pub fn debug_info(message: &str) {
    println!(
        "{} {}",
        "[UUI-INFO]:".cyan(),
        message
    )
}

//  warning, no significant issues to functionality but
//  non ideal implementation.
pub fn debug_warning(message: &str) {
    println!(
        "{} {}",
        "[UUI-WARNING]:".yellow(),
        message
    )
}

//  error, wrong implementation and functionality affected.
//  Typically used when using a function incorrectly.
pub fn debug_error(message: &str) {
    println!(
        "{} {}",
        "[UUI-ERROR]:".bright_yellow(),
        message
    )
}

//  critical error, app about to crash
pub fn debug_critical(message: &str) {
    println!(
        "{} {}",
        "[UUI-CRITICAL]:".bright_red(),
        message
    )
}