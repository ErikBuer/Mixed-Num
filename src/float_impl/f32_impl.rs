use super::*;

impl MixedSin for f32
{
    #[inline(always)]
    fn mixed_sin(&self) -> Self {
        return libm::sinf(*self);
    }
    #[inline(always)]
    fn mixed_sincos(&self) -> (Self, Self) 
        where Self: Sized
    {
        return libm::sincosf(*self);
    }   
    #[inline(always)]
    fn mixed_asin(&self) -> Self {
        return libm::asinf(*self);
    }
}

impl MixedSinh for f32
{
    #[inline(always)]
    fn mixed_sinh(&self) -> Self {
        return libm::sinhf(*self);
    }
    #[inline(always)]
    fn mixed_asinh(&self) -> Self {
        return libm::asinhf(*self);
    }
}

impl MixedCos for f32
{
    #[inline(always)]
    fn mixed_cos(&self) -> Self {
        return libm::cosf(*self);
    }
    #[inline(always)]
    fn mixed_acos(&self) -> Self {
        return libm::acosf(*self);
    }
}

impl MixedCosh for f32
{
    #[inline(always)]
    fn mixed_cosh(&self) -> Self {
        return libm::coshf(*self);
    }
    #[inline(always)]
    fn mixed_acosh(&self) -> Self {
        return libm::acoshf(*self);
    }
}

impl MixedTrigonometry for f32
{ 
}

impl MixedTan for f32
{
    #[inline(always)]
    fn mixed_tan(&self) -> Self {
        return libm::tanf(*self);
    }
}

impl MixedTanh for f32
{
    #[inline(always)]
    fn mixed_tanh(&self) -> Self {
        return libm::tanhf(*self);
    }
    #[inline(always)]
    fn mixed_atanh(&self) -> Self {
        return libm::atanhf(*self);
    }
}

impl MixedAtan for f32
{
    #[inline(always)]
    fn mixed_atan(&self) -> Self {
        return libm::atanf(*self);
    }
    #[inline(always)]
    fn mixed_atan2(&self, other:Self) -> Self {
        return libm::atan2f(*self,other);
    }
    #[inline(always)]
    fn mixed_atan2_poly(&self, other:Self) -> Self {
        return trigonometry::atan::atan2(*self, other);
    }
}

impl MixedSqrt for f32
{
    #[inline(always)]
    fn mixed_sqrt(&self) -> Self {
        return libm::sqrtf(*self);
    }

    #[inline(always)]
    fn mixed_niirf(&self) -> Self {
        return trigonometry::sqrt::niirf(*self, 2);
    }
}

impl MixedCbrt for f32
{
    #[inline(always)]
    fn mixed_cbrt(&self) -> Self {
        return libm::cbrtf(*self);
    }
}

impl MixedCeil for f32
{
    #[inline(always)]
    fn mixed_ceil(&self) -> Self {
        return libm::ceilf(*self);
    }
}
impl MixedFloor for f32
{
    #[inline(always)]
    fn mixed_floor(&self) -> Self {
        return libm::floorf(*self);
    }
}

impl MixedExp for f32
{
    #[inline(always)]
    fn mixed_exp(&self) -> Self {
        return libm::expf(*self);
    }
}

impl MixedExp10 for f32
{
    #[inline(always)]
    fn mixed_exp10(&self) -> Self {
        return libm::exp10f(*self);
    }
}

impl MixedExp2 for f32
{
    #[inline(always)]
    fn mixed_exp2(&self) -> Self {
        return libm::exp2f(*self);
    }
}

impl MixedPow for f32
{
    #[inline(always)]
    fn mixed_pow(&self, power:f32) -> Self {
        return libm::powf(*self, power);
    }
}

impl Mixedlog for f32
{
    #[inline(always)]
    fn mixed_log(&self) -> Self {
        return libm::logf(*self);
    }
}

impl Mixedlog10 for f32
{
    #[inline(always)]
    fn mixed_log10(&self) -> Self {
        return libm::log10f(*self);
    }
}

impl Mixedlog2 for f32
{
    #[inline(always)]
    fn mixed_log2(&self) -> Self {
        return libm::log2f(*self);
    }
}