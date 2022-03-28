//! No-STD abstraction layer enabling numerical functions to be implemented once, and simultaneously support both real and complex numbers with, int, fixed and floating point types.
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
//! 
//! 
//! The library supplies complex structs `Polar<T>` and `Cartesian<T>` with no-std implementation of math traits, including `MixedNum` traits.
//! 
//! Some interoperability with num::Complex is implemented.
//! 
//! ## Example
//! 
//! ```
//! use mixed_num::*;
//! use mixed_num::traits::*;
//! 
//! let number = Cartesian::new(1f32,2f32);
//! 
//! assert_eq!{ number.to_string(), "1+2i" };
//! 
//! let polar_number = number.to_polar();
//! assert_eq!{ polar_number.to_string(), "2.236068∠1.1071488" };
//! 
//! let polar_conj: Polar<f32> = polar_number.conj();
//! 
//! assert_eq!{ polar_conj.to_string(), "2.236068∠-1.1071488" };
//! ``` 
//! 
//! Selected `core::ops` traits are implemented on the complex structs.
//! 
//! ## Example
//! 
//! ```
//! use mixed_num::*;
//! use mixed_num::traits::*;
//! 
//! let mut c_num = Cartesian::new(1f32,2f32);
//! 
//! c_num = c_num*c_num;
//! assert_eq!{ c_num.to_string(), "-3+4i" };
//! ```
//! 
//! This includes support for operation of mixed types.
//! 
//! ## Example
//! 
//! ```
//! use mixed_num::*;
//! use mixed_num::traits::*;
//! 
//! let mut c_num = Cartesian::new(1f32,2f32);
//! 
//! c_num = c_num*2f64;
//! assert_eq!{ c_num.to_string(), "2+4i" };
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

mod utility;
