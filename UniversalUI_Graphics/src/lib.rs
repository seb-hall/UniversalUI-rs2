//  UniversalUI_Graphics - lib.rs
//  created by sebhall on 30/07/2023
//  
//  UniversalUI_Graphics provides graphical functionality
//  to the UniversalUI framework. It intends to allow for
//  both 2D and 3D rendering, working seamlessly with the
//  UniversalUI_Native windowing and events library.
//
//  lib.rs declares the crate submodules and init function.

extern crate UniversalUI_Base;
extern crate libc;

use UniversalUI_Base::debug::*;


#[no_mangle]
pub extern "C" fn graphics_init() -> bool { 

    debug_info("Initialising UniversalUI_Graphics");
    
    return true;
}