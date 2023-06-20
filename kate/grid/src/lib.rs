#![no_std]
#![deny(clippy::integer_arithmetic)]
//! Nice grid API, dealing with grids of different sizes and different orders
//! (column-major/row-major)

#[cfg_attr(test, macro_use)]
extern crate alloc;

mod grid;
pub use grid::*;
