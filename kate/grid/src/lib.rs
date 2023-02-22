#![no_std]
//! Nice grid API, dealing with grids of different sizes and different orders
//! (column-major/row-major)

#[cfg_attr(test, macro_use)]
extern crate alloc;

mod dims;
mod grid;
pub use dims::*;
pub use grid::*;
