//! 
//! No-STD abstraction layer enabling numerical functions to be implemented once, and simultaneously support both fixed and floating point types.
//! The crate focueses on computationally efficient implementations of numerical operations.
//! 
//! This is an experimental library.
//! 

#![crate_name = "mixed_num"]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod trigonometry;

pub mod traits;
use traits::*;

mod float_impl;
pub use float_impl::*;

mod fixed_impl;
pub use fixed_impl::*;