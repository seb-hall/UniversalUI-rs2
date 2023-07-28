//  UniversalUI_Base - general.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Base contains definitions for types
//  common to other modules of the UniversalUI framework.
//
//  general.rs contains base types useful for modules within
//  UniversalUI_Base and beyond.

use std::ffi::c_float;

//  uID - used as an ID for various framework elements.
pub type uID = usize;

//  uFloat - a standard floating point number.
pub type uFloat = c_float;