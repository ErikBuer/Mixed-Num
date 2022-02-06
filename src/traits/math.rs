/* 
acos	Arccosine (f64)
acosf	Arccosine (f32)
acosh	Inverse hyperbolic cosine (f64)
acoshf	Inverse hyperbolic cosine (f32)
asin	Arcsine (f64)
asinf	Arcsine (f32)
asinh	Inverse hyperbolic sine (f64)
asinhf	Inverse hyperbolic sine (f32)
atan	Arctangent (f64)
atan2	Arctangent of y/x (f64)
atan2f	Arctangent of y/x (f32)
atanf	Arctangent (f32)
atanh	Inverse hyperbolic tangent (f64)
atanhf	Inverse hyperbolic tangent (f32)
cbrt	Computes the cube root of the argument.
cbrtf	Cube root (f32)
ceil	Ceil (f64)
ceilf	Ceil (f32)
copysign	Sign of Y, magnitude of X (f64)
copysignf	Sign of Y, magnitude of X (f32)
cos	
cosf	
cosh	Hyperbolic cosine (f64)
coshf	Hyperbolic cosine (f64)
erf	Error function (f64)
erfc	Error function (f64)
erfcf	Error function (f32)
erff	Error function (f32)
exp	Exponential, base e (f64)
exp2	Exponential, base 2 (f64)
exp2f	Exponential, base 2 (f32)
exp10	
exp10f	
expf	Exponential, base e (f32)
expm1	Exponential, base e, of x-1 (f64)
expm1f	Exponential, base e, of x-1 (f32)
fabs	Absolute value (magnitude) (f64) Calculates the absolute value (magnitude) of the argument x, by direct manipulation of the bit representation of x.
fabsf	Absolute value (magnitude) (f32) Calculates the absolute value (magnitude) of the argument x, by direct manipulation of the bit representation of x.
fdim	Positive difference (f64)
fdimf	Positive difference (f32)
floor	Floor (f64)
floorf	Floor (f32)
fma	Floating multiply add (f64)
fmaf	Floating multiply add (f32)
fmax	
fmaxf	
fmin	
fminf	
fmod	
fmodf	
frexp	
frexpf	
hypot	
hypotf	
ilogb	
ilogbf	
j0	
j0f	
j1	
j1f	
jn	
jnf	
ldexp	
ldexpf	
lgamma	
lgamma_r	
lgammaf	
lgammaf_r	
log	
log1p	
log1pf	
log2	
log2f	
log10	
log10f	
logf	
modf	
modff	
nextafter	
nextafterf	
pow	
powf	
remainder	
remainderf	
remquo	
remquof	
round	
roundf	
scalbn	
scalbnf	
sin	
sincos	
sincosf	
sinf	
sinh	
sinhf	
sqrt	
sqrtf	
tan	
tanf	
tanh	
tanhf	
tgamma	
tgammaf	
trunc	
truncf	
y0	
y0f	
y1	
y1f	
yn	
ynf
*/


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

pub trait MixedCos
{
    /// Take the cos of `self`. Implementation varies with type.
    fn mixed_cos(&self) -> Self;
    /// Take the arccos of `self`. Implementation varies with type.
    fn mixed_acos(&self) -> Self;
}

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