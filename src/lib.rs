//! 
//! No-STD abstraction layer enabling numerical functions to be implemented once, and simultaneously support both real and complex numbers with, int, fixed and floating point types.
//! 
//! The library supplies complex structs `Polar<T>` and `Cartesian<T>` with no-std implementation of math traits, including `MixedNum` traits.
//! 
//! Some interoperability with num::Complex is implemented.
//! 
//! This is an experimental library.
//! 
//! ## Example
//! 
//! ```
//! use mixed_num::*;
//! use mixed_num::traits::*;
//! use fixed::{types::extra::U27, FixedI32};
//! 
//! let number = FixedI32::<U27>::from_num(0.6f32);
//! let res:f32 = number.mixed_atan().mixed_to_num();
//! 
//! assert_eq!{ res, 0.5404195 };
//! 
//! let number = 0.6f32;
//! let res:f32 = number.mixed_atan().mixed_to_num();
//! 
//! assert_eq!{ res, 0.5404195 };
//! ``` 

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

pub mod complex;
#[allow(unused)]
pub use complex::*;

mod int_impl;
pub use int_impl::*;
