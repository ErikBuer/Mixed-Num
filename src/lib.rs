//! 
//! No-STD abstraction layer enabling numerical functions to be implemented once, and simultaneously support both fixed and floating point types.
//! The crate focueses on computationally efficient implementations of numerical operations.
//! 
//! This is an experimental library.
//! 

#![crate_name = "mixed_num"]
#![no_std]

use fixed;
use num::traits::float::FloatCore;
use fixed_trigonometry as fixed_trig;

mod trigonometry;

pub trait MixedNumConversion<T> {
    /// Generic type cast from numeric type T.
    fn mixed_from_num( number:T ) -> Self;
    /// Generic type cast to  numeric type T.
    fn mixed_to_num( &self )      -> T;
}

pub trait MixedTrigonometry
{
    /// Take the sin of `self`. Implementation varies with type.
    fn mixed_sin(&self) -> Self;
    /// Take the sin of `self`. Implementation varies with type.
    fn mixed_cos(&self) -> Self;
    /// Take the sin of `self`. Implementation varies with type.
    fn mixed_atan(&self) -> Self;
    /// Take the atan2 of `self`/other. Implementation varies with type.
    fn mixed_atan2(&self, other:Self) -> Self;
}

pub trait MixedWrapPhase
{
    /// Wrapps `self` to the -π=<x<π range.
    fn mixed_wrap_phase(&self) -> Self;
}

pub trait MixedConsts
{
    /// The mixed_pi constant. 3.141...
    fn mixed_pi() -> Self;
}

pub trait MixedSqrt
{
    /// The generic square root implementation for the `MixedSqrt` trait. The implimentation may change with time.
    /// To stay on this excact implementation, use the `mixed_niirf` function.
    fn mixed_sqrt(&self) -> Self;
    /// A fast implementation of the square root using the Nonlinear IIR Filter (NIIRF) method \[1\].
    /// 
    /// Only valid for positive values of `self`. Negative values are forced positive before converison.
    /// Accurate to 5*10⁻⁴ with two iterations \[2\].
    /// 
    /// The structure of the estimator is illustrated below \[1\].
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/ErikBuer/Fixed-Trigonometry/main/figures/niirf.svg)
    /// 
    /// The method utilizes a lookup-table for the acceleration factor β.
    /// 
    /// β(x) can be calculated from the following formula, yielding even greater accuracy at a computational cost.
    /// ```Julia
    /// β(x) = 0.763x^2-1.5688x+1.314 
    /// ```
    /// 
    /// \[1\] N.Mikami et al., A new DSP-oriented algorithm for calculation of square root using a non-linear digital filter, IEEE Trans. on Signal Processing, July 1992, pp. 1663-1669.
    /// 
    /// \[2\] R. G. Lyons, Streamlining Digital Signal Processing, Second Edition, IEEE Press, 2012.
    /// 
    /// 
    /// ## Accuracy and Comparison
    /// 
    /// The figure below shows error of the NIIRF implementation, compared to the `std::f32::sqrt` implementation.
    /// 
    /// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/niirf_sqrt_comparison.png?raw=true)
    /// 
    /// Another fixed point implementation of the square root can be found in the cordic crate. 
    /// 
    /// Below is the error comparison between the two implementations.
    /// 
    /// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/sqrt_error_comparison.png?raw=true)
    fn mixed_niirf(&self) -> Self;
}

pub trait MixedNum
    where Self: MixedConsts 
                + MixedNumConversion<i32> + MixedNumConversion<i64>
                + MixedNumConversion<f32> + MixedNumConversion<f64>
                + core::cmp::PartialOrd
                + core::marker::Sized
                + core::ops::Div<Output = Self>
                + core::ops::Sub<Output = Self>
                + core::ops::Add<Output = Self>
                + core::ops::Mul<Output = Self>
                + core::ops::AddAssign
                + core::ops::SubAssign
                + Copy
{
    /// Maximum value of the type.
    fn mixed_max_value() -> Self;
    /// Minimum value of the type.
    fn mixed_min_value() -> Self;
    /// Absolute value.
    fn mixed_abs( &self ) -> Self;
    /// Integer valued power.
    fn mixed_powi( &self, exp: i32 ) -> Self;
    /// Get the sign of the argument with a unit value.
    /// Zero is of positive sign.
    fn mixed_sign( &self ) -> Self;
    /// Returns a bool if self is positive.
    fn mixed_is_positive( &self ) -> bool;
    /// Returns a bool if self is negative.
    fn mixed_is_negative( &self ) -> bool;
}

pub trait MixedNumSigned
    where Self: core::ops::Neg<Output = Self>
{
}

macro_rules! impl_mixed_num_for_primitive{
    ( $T:ty ) => {

        impl MixedNumConversion<f32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:f32 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> f32 {
                return *self as f32;
            }
        }

        impl MixedNumConversion<f64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:f64 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> f64 {
                return *self as f64;
            }
        }

        impl MixedNumConversion<i32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:i32 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> i32 {
                return *self as i32;
            }
        }

        impl MixedNumConversion<i64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:i64 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> i64 {
                return *self as i64;
            }
        }

        impl MixedTrigonometry for $T
        {
            /// Take the sin of self.
            #[inline(always)]
            fn mixed_sin(&self) -> Self {
                return trigonometry::sin(*self);
            }
            /// Take the cos of self.
            #[inline(always)]
            fn mixed_cos(&self) -> Self {
                return trigonometry::cos(*self);
            }
            /// Take the atan of self.
            #[inline(always)]
            fn mixed_atan(&self) -> Self {
                return trigonometry::atan::atan(*self);
            }
            /// Take the atan of self.
            #[inline(always)]
            fn mixed_atan2(&self, other:Self) -> Self {
                return trigonometry::atan::atan2(*self, other);
            }
        }

        impl MixedSqrt for $T
        {
            /// Take the square root of self.
            #[inline(always)]
            fn mixed_sqrt(&self) -> Self {
                return trigonometry::sqrt::niirf(*self, 2);
            }
            /// Take the square root of self.
            #[inline(always)]
            fn mixed_niirf(&self) -> Self {
                return trigonometry::sqrt::niirf(*self, 2);
            }
        }

        impl MixedWrapPhase for $T
        {
            #[inline(always)]
            fn mixed_wrap_phase(&self) -> Self {
                return trigonometry::wrap_phase(*self);
            }
        }

        impl MixedNumSigned for $T
        {   
        }

        impl MixedNum for $T
        {
            #[inline(always)]
            fn mixed_max_value() -> Self {
                return Self::max_value();
            }
            #[inline(always)]
            fn mixed_min_value() -> Self {
                return Self::min_value();
            }
            #[inline(always)]
            fn mixed_abs( &self ) -> Self {
                return self.abs();
            }
            #[inline(always)]
            fn mixed_powi( &self, exp: i32 ) -> Self {
                return self.powi( exp );
            }
            fn mixed_sign( &self) -> Self {
                return trigonometry::sign(*self);
            }
            #[inline(always)]
            fn mixed_is_positive( &self) -> bool {
                return self.is_sign_positive();
            }
            #[inline(always)]
            fn mixed_is_negative( &self) -> bool {
                return self.is_sign_negative();
            }
        }

        impl MixedConsts for $T
        {
            #[inline(always)]
            fn mixed_pi() -> Self {
                return 3.1415926535897932384626433832795028841971693993751058209749445923078164062 as $T;
            }
        }
    }
}

impl_mixed_num_for_primitive!(f32);
impl_mixed_num_for_primitive!(f64);

macro_rules! impl_mixed_num_for_fixed{
    ( $T:ty ) => {

        impl MixedNumConversion<f32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:f32 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> f32 {
                return self.to_num::<f32>();
            }
        }

        impl MixedNumConversion<f64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:f64 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> f64 {
                return self.to_num::<f64>();
            }
        }

        impl MixedNumConversion<i32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:i32 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> i32 {
                return self.to_num::<i32>();
            }
        }

        impl MixedNumConversion<i64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:i64 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> i64 {
                return self.to_num::<i64>();
            }
        }

        impl MixedConsts for $T
        {
            #[inline(always)]
            fn mixed_pi() -> Self {
                return Self::from_num(3.1415926535897932384626433832795028841971693993751058209749445923078164062);
            }
        }

        impl MixedSqrt for $T
        {
            /// Take the square root of self.
            #[inline(always)]
            fn mixed_sqrt(&self) -> Self {
                return trigonometry::sqrt::niirf(*self, 2);
            }
            /// Take the square root of self.
            #[inline(always)]
            fn mixed_niirf(&self) -> Self {
                return trigonometry::sqrt::niirf(*self, 2);
            }
        }
    }
}

macro_rules! impl_mixed_num_for_fixed_unsigned{
    ( $T:ty ) => {
        impl_mixed_num_for_fixed!($T);

        impl MixedNum for $T
        {
            #[inline(always)]
            fn mixed_max_value() -> Self {
                return Self::MAX;
            }
            #[inline(always)]
            fn mixed_min_value() -> Self {
                return Self::MIN;
            }
            #[inline(always)]
            fn mixed_abs( &self ) -> Self {
                return *self; // Is itself for unsigned.
            }
            #[inline(always)]
            fn mixed_powi( &self, exp: i32 ) -> Self {
                return fixed_trig::powi( *self, exp as usize );
            }
            #[inline(always)]
            fn mixed_sign( &self) -> Self {
                return trigonometry::sign(*self);
            }
            #[inline(always)]
            fn mixed_is_positive( &self) -> bool {
                return true;
            }
            #[inline(always)]
            fn mixed_is_negative( &self) -> bool {
                return false;
            }
        }
    }
}

macro_rules! impl_mixed_num_for_fixed_signed{
    ( $T:ty ) => {
        impl_mixed_num_for_fixed!($T);

        impl MixedNumSigned  for $T
        {
        }

        impl MixedNum for $T
        {
            #[inline(always)]
            fn mixed_max_value() -> Self {
                return Self::MAX;
            }
            #[inline(always)]
            fn mixed_min_value() -> Self {
                return Self::MIN;
            }
            #[inline(always)]
            fn mixed_abs( &self ) -> Self {
                return self.abs();
            }
            #[inline(always)]
            fn mixed_powi( &self, exp: i32 ) -> Self {
                return fixed_trig::powi( *self, exp as usize );
            }
            #[inline(always)]
            fn mixed_sign( &self) -> Self {
                return trigonometry::sign(*self);
            }
            #[inline(always)]
            fn mixed_is_positive( &self) -> bool {
                return self.is_positive();
            }
            #[inline(always)]
            fn mixed_is_negative( &self) -> bool {
                return self.is_negative();
            }
        }

        impl MixedWrapPhase for $T
        {
            #[inline(always)]
            fn mixed_wrap_phase(&self) -> Self {
                return trigonometry::wrap_phase(*self);
            }
        }

        impl MixedTrigonometry for $T
        {
            /// Take the sin of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_sin(&self) -> Self {
                return fixed_trig::sin(*self);
            }
            /// Take the cos of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_cos(&self) -> Self {
                return fixed_trig::cos(*self);
            }
            /// Take the atan of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_atan(&self) -> Self {
                return trigonometry::atan::atan(*self);
            }
            /// Take the atan of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_atan2(&self, other:Self) -> Self {
                return trigonometry::atan::atan2(*self, other);
            }
        }
    }
}

impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U8>);

impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U8>);


impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U16>);

impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U16>);


impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U16>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U17>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U18>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U19>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U20>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U21>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U22>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U23>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U24>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U25>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U26>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U27>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U28>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U29>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U30>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U31>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U32>);

impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U16>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U17>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U18>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U19>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U20>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U21>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U22>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U23>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U24>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U25>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U26>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U27>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U28>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U29>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U30>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U31>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U32>);


impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U16>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U17>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U18>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U19>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U20>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U21>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U22>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U23>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U24>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U25>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U26>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U27>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U28>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U29>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U30>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U31>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U32>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U33>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U34>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U35>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U36>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U37>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U38>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U39>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U40>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U41>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U42>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U43>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U44>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U45>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U46>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U47>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U48>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U49>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U50>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U51>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U52>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U53>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U54>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U55>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U56>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U57>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U58>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U59>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U60>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U61>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U62>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U63>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U64>);

impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U16>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U17>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U18>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U19>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U20>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U21>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U22>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U23>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U24>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U25>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U26>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U27>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U28>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U29>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U30>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U31>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U32>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U33>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U34>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U35>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U36>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U37>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U38>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U39>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U40>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U41>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U42>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U43>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U44>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U45>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U46>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U47>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U48>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U49>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U50>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U51>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U52>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U53>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U54>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U55>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U56>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U57>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U58>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U59>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U60>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U61>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U62>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U63>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U64>);