use crate::*;

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

pub trait MixedWrapPhase
{
    /// Wrapps `self` to the -π=<x<π range.
    fn mixed_wrap_phase(&self) -> Self;
}

pub trait MixedTan
{
    /// Take the tan of `self`. Implementation varies with type.
    fn mixed_tan(&self) -> Self;
}

pub trait MixedTanh
{
    /// Take the hyperbolic tangent (tanh) of `self`. Implementation varies with type.
    fn mixed_tanh(&self) -> Self;
    /// Take the inverse hyperbolic tangent (atanh) of `self`. Implementation varies with type.
    fn mixed_atanh(&self) -> Self;
}

pub trait MixedAtan
{
    /// Take the atan of `self`. Implementation varies with type.
    fn mixed_atan(&self) -> Self;
    /// Take the atan2 of `self`/other. Implementation varies with type.
    fn mixed_atan2(&self, other:Self) -> Self;
    /// Calculate atan2(y,x) using a selection of polynomial approximations, one for each octant in the unit circle.
    /// 
    /// The method is accurat within 0.028 degrees.
    /// 
    /// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Edition, IEEE Press, 2012.
    /// 
    /// ## Comparisons
    /// 
    /// The figure below shows the comparison between the various implementations and the `std::f32::atan` implementation.
    /// 
    /// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/atan2_comparisons.png?raw=true)
    /// 
    fn mixed_atan2_poly(&self, other:Self) -> Self;
}

pub trait MixedSin
{
    /// Take the sin of `self`. Implementation varies with type.
    fn mixed_sin(&self) -> Self;
    /// Calculate the sin and cos of `self`. Implementation varies with type.
    fn mixed_sincos(&self) -> (Self, Self) where Self:Sized;
    /// Take the arcsin of `self`. Implementation varies with type.
    fn mixed_asin(&self) -> Self;
}

pub trait MixedSinh
{
    /// Take the hyperbolic sin of `self`. Implementation varies with type.
    fn mixed_sinh(&self) -> Self;
    /// Take the inverse hyperbolic sin of `self`. Implementation varies with type.
    fn mixed_asinh(&self) -> Self;
}

pub trait MixedCos
{
    /// Take the cos of `self`. Implementation varies with type.
    fn mixed_cos(&self) -> Self;
    /// Take the arccos of `self`. Implementation varies with type.
    fn mixed_acos(&self) -> Self;
}

pub trait MixedCosh
{
    /// Take the cosh of `self`. Implementation varies with type.
    fn mixed_cosh(&self) -> Self;
    /// Take the arccosh of `self`. Implementation varies with type.
    fn mixed_acosh(&self) -> Self;
}

// Trait kept for legacy reasons
pub trait MixedTrigonometry: MixedSin + MixedCos + MixedAtan
{
}

pub trait MixedExp
{
    /// Take the exponential, base e, of `self`.
    fn mixed_exp(&self) -> Self;
}

pub trait MixedSqrt
{
    /// The generic square root implementation for the `MixedSqrt` trait.
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

pub trait MixedCbrt
{
    /// Take the cube root of self.
    fn mixed_cbrt(&self) -> Self;
}

pub trait MixedExp10
{
    /// Take the exponential, base 10, of `self`.
    fn mixed_exp10(&self) -> Self;
}

pub trait MixedExp2
{
    /// Take the exponential, base 10, of `self`.
    fn mixed_exp2(&self) -> Self;
}

pub trait MixedPow
{
    /// Take the exponential, base 10, of `self`.
    fn mixed_pow(&self, power:Self) -> Self;
}