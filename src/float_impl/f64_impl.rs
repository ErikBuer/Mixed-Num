use super::*;

impl MixedSin for f64
{
    #[inline(always)]
    fn mixed_sin(&self) -> Self {
        return libm::sin(*self);
    }
    #[inline(always)]
    fn mixed_sincos(&self) -> (Self, Self) 
        where Self: Sized
    {
        return libm::sincos(*self);
    }   
    #[inline(always)]
    fn mixed_asin(&self) -> Self {
        return libm::asin(*self);
    }
}

impl MixedSinh for f64
{
    #[inline(always)]
    fn mixed_sinh(&self) -> Self {
        return libm::sinh(*self);
    }
    #[inline(always)]
    fn mixed_asinh(&self) -> Self {
        return libm::asinh(*self);
    }
}

impl MixedCos for f64
{
    #[inline(always)]
    fn mixed_cos(&self) -> Self {
        return libm::cos(*self);
    }
    #[inline(always)]
    fn mixed_acos(&self) -> Self {
        return libm::acos(*self);
    }
}

impl MixedCosh for f64
{
    #[inline(always)]
    fn mixed_cosh(&self) -> Self {
        return libm::cosh(*self);
    }
    #[inline(always)]
    fn mixed_acosh(&self) -> Self {
        return libm::acosh(*self);
    }
}

impl MixedTrigonometry for f64
{ 
}

impl MixedTan for f64
{
    #[inline(always)]
    fn mixed_tan(&self) -> Self {
        return libm::tan(*self);
    }
}

impl MixedTanh for f64
{
    #[inline(always)]
    fn mixed_tanh(&self) -> Self {
        return libm::tanh(*self);
    }
    #[inline(always)]
    fn mixed_atanh(&self) -> Self {
        return libm::atanh(*self);
    }
}

impl MixedCbrt for f64
{
    #[inline(always)]
    fn mixed_cbrt(&self) -> Self {
        return libm::cbrt(*self);
    }
}

impl MixedAtan for f64
{
    #[inline(always)]
    fn mixed_atan(&self) -> Self {
        return libm::atan(*self);
    }
    #[inline(always)]
    fn mixed_atan2(&self, other:Self) -> Self {
        return libm::atan2(*self,other);
    }
    #[inline(always)]
    fn mixed_atan2_poly(&self, other:Self) -> Self {
        return trigonometry::atan::atan2(*self, other);
    }
}

impl MixedSqrt for f64
{
    /// Calculate the square root of self.
    /// 
    /// ```
    /// use mixed_num::*;
    /// 
    /// let mut num = 4f64;
    /// num = num.mixed_sqrt();
    /// assert_eq!(num, 2f64 );
    /// 
    /// let mut num = 0f64;
    /// num = num.mixed_sqrt();
    /// assert_eq!(num, 0f64 );
    /// ``` 
    #[inline(always)]
    fn mixed_sqrt(&self) -> Self {
        return libm::sqrt(*self);
    }

    #[inline(always)]
    fn mixed_niirf(&self) -> Self {
        return trigonometry::sqrt::niirf(*self, 2);
    }
}

impl MixedCeil for f64
{
    #[inline(always)]
    fn mixed_ceil(&self) -> Self {
        return libm::ceil(*self);
    }
}
impl MixedFloor for f64
{
    #[inline(always)]
    fn mixed_floor(&self) -> Self {
        return libm::floor(*self);
    }
}

impl MixedExp for f64
{
    #[inline(always)]
    fn mixed_exp(&self) -> Self {
        return libm::exp(*self);
    }
}
impl MixedExp10 for f64
{
    #[inline(always)]
    fn mixed_exp10(&self) -> Self {
        return libm::exp10(*self);
    }
}

impl MixedExp2 for f64
{
    #[inline(always)]
    fn mixed_exp2(&self) -> Self {
        return libm::exp2(*self);
    }
}

impl MixedPow for f64
{
    #[inline(always)]
    fn mixed_pow(&self, power:f64) -> Self {
        return libm::pow(*self, power);
    }
}

impl Mixedlog for f64
{
    #[inline(always)]
    fn mixed_log(&self) -> Self {
        return libm::log(*self);
    }
}

impl Mixedlog10 for f64
{
    /// Calculate the log10 of self.
    /// 
    /// ```
    /// use mixed_num::*;
    /// 
    /// let mut num = 10f64;
    /// num = num.mixed_log10();
    /// assert_eq!(num, 1f64 );
    /// 
    /// let mut num = 1f64;
    /// num = num.mixed_log10();
    /// assert_eq!(num, 0f64 );
    /// 
    ///  let mut num = 0f64;
    /// num = num.mixed_log10();
    /// assert_eq!(num.to_string(), "-inf" );
    /// ``` 
    #[inline(always)]
    fn mixed_log10(&self) -> Self {
        return libm::log10(*self);
    }
}

impl Mixedlog2 for f64
{
    #[inline(always)]
    fn mixed_log2(&self) -> Self {
        return libm::log2(*self);
    }
}