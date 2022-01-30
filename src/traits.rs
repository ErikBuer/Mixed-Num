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
}

pub trait MixedConsts
    where Self: MixedPi + MixedZero + MixedOne
{
}

pub trait MixedSqrt
{
    /// The generic square root implementation for the `MixedSqrt` trait.
    /// When the STD feature is enabled, its sqrt implementation is used fro f32 and f64 types.
    /// When not, the Nonlinear IIR Filter (NIIRF) method is implemented.
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