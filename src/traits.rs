mod math;
pub use math::*;

pub trait MixedNumConversion<T> {
    /// Generic type cast from numeric type T.
    fn mixed_from_num( number:T ) -> Self;
    /// Generic type cast to  numeric type T.
    fn mixed_to_num( &self )      -> T;
}

pub trait MixedWrapPhase
{
    /// Wrapps `self` to the -π=<x<π range.
    fn mixed_wrap_phase(&self) -> Self;
}

pub trait MixedZero
{
    /// Return the zero value of type Self.
    fn mixed_zero() -> Self;
}

pub trait MixedOne
{
    /// Return the zero value of type Self.
    fn mixed_one() -> Self;
}

pub trait MixedPi
{
    /// The mixed_pi constant. 3.141...
    fn mixed_pi() -> Self;
    fn mixed_tau() -> Self;
}

pub trait MixedConsts
    where Self: MixedPi + MixedZero + MixedOne
{
}
pub trait MixedOps
    where Self: MixedConsts 
                + MixedNumConversion<i32> + MixedNumConversion<i64>
                + MixedNumConversion<f32> + MixedNumConversion<f64>
                + core::cmp::PartialOrd
                + core::marker::Sized
                + core::ops::AddAssign
                + core::ops::SubAssign
                + num::traits::NumOps
                + Copy
{
    /// Absolute value.
    fn mixed_abs( &self ) -> Self;
    /// Integer valued power.
    fn mixed_powi( &self, exp: i32 ) -> Self;
}

pub trait MixedNum
    where Self: MixedConsts 
                + MixedOps
                + MixedNumConversion<u32> + MixedNumConversion<u64>
                + MixedNumConversion<i32> + MixedNumConversion<i64>
                + MixedNumConversion<f32> + MixedNumConversion<f64>
                + core::cmp::PartialOrd
                + core::marker::Sized
                + Copy
{
    /// Maximum value of the type.
    fn mixed_max_value() -> Self;
    /// Minimum value of the type.
    fn mixed_min_value() -> Self;
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