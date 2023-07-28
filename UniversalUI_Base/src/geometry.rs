//  UniversalUI_Base - geometry.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Base contains definitions for types
//  common to other modules of the UniversalUI framework.
//
//  geometry.rs contains types associated with 2D
//  geometry, particularly for windowing and basic
//  graphics drawing.

use crate::general::*;

#[allow(non_camel_case_types)]
//  uPoint, a simple location in 2D space
pub struct uPoint {
    pub x: uFloat,
    pub y: uFloat,
}

#[allow(non_camel_case_types)]
//  uSize, a 2D orthogonal size
pub struct uSize {
    pub width: uFloat,
    pub height: uFloat,
}

#[allow(non_camel_case_types)]
//  uRect, a 2D orthogonal rectangle
pub struct uRect {
    pub x: uFloat,
    pub y: uFloat,
    pub width: uFloat,
    pub height: uFloat,
}
