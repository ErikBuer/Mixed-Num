use super::*;

impl <T: MixedNum + MixedNumSigned> MixedComplex for Complex<T>
{
}

impl<T> Complex<T>
{
    pub fn new(re:T, im:T) -> Self
    {
        return Complex{re:re, im:im};
    }
}

impl <T: MixedNum + MixedNumSigned> NewFromCartesian<T> for Complex<T>
{
    /// Type cast from real number T to Complex<T>.
    fn new_from_cartesian( re:T, im:T ) -> Self
    {
        return Self{re:re, im:im};
    }
}

impl <T: MixedNum + MixedNumSigned + MixedWrapPhase + MixedTrigonometry + MixedOps> NewFromPolar<T> for Complex<T>
{
    fn new_from_polar( mag:T, ang:T ) -> Self
    {
        return ops::to_cartsian(Polar::new(mag, ang));
    }
}

impl <T: MixedNum + MixedNumConversion<T2> + MixedZero, T2: MixedNum> MixedNumConversion<T2> for Complex<T>
{
    #[inline(always)]
    fn mixed_from_num( number:T2 ) -> Self {
        return Self{re:T::mixed_from_num(number), im:T::mixed_zero()};
    }
    #[inline(always)]
    fn mixed_to_num( &self ) -> T2 {
        return self.re.mixed_to_num();
    }
}

impl <T: MixedNum + MixedNumSigned> ToComplex<T> for Complex<T>
{
    /// Complex<T> to Polar<T>.
    #[inline(always)]
    fn to_complex( &self ) -> Complex<T>
    {
        return *self;
    }
}

impl <T: MixedNum + MixedNumSigned + MixedSqrt + MixedOps + MixedAbs + MixedPowi + MixedAtan> ToPolar<T> for Complex<T>
{
    /// Complex<T> to Polar<T>.
    #[inline(always)]
    fn to_polar( &self ) -> Polar<T>
    {
        return ops::to_polar(*self);
    }
}

impl <T: MixedNum + MixedZero> MixedZero for Complex<T>
{
    /// Return the zero value of type Self.
    #[inline(always)]
    fn mixed_zero() -> Self {
        return Self{re:T::mixed_zero(), im:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedZero + MixedOne> MixedOne for Complex<T>
{
    /// Return the zero value of type Self.
    #[inline(always)]
    fn mixed_one() -> Self {
        return Self{re:T::mixed_one(), im:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedZero> MixedComplexConversion<T>  for Complex<T>
{
    /// Type cast from real number T to Complex<T>.
    fn mixed_to_complex( number:T ) -> Self {
        return Self{re:number, im:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedNumSigned + MixedSqrt + MixedPowi + MixedOps> Mag<T> for Complex<T>
{
    /// Magnitude of the complex number.
    #[inline(always)]
    fn mag( &self ) -> T
    {
        return (self.re.mixed_powi(2)+self.im.mixed_powi(2)).mixed_sqrt();
    }
    /// Magnitude of the complex number.
    #[inline(always)]
    fn abs( &self ) -> T
    {
        return self.mag();
    }
}

impl <T: MixedNum + MixedNumSigned + MixedSqrt + MixedPowi + MixedOps + MixedZero> MixedAbs for Complex<T>
{
    fn mixed_abs( &self ) -> Self
    {
        return Self::new(self.mag(), T::mixed_zero());
    }
}

impl <T: MixedNum + MixedNumSigned + MixedAtan> Arg<T> for Complex<T>
{
    /// Argument of the complex number.
    #[inline(always)]
    fn arg( &self ) -> T
    {
        return self.re.mixed_atan2(self.im);
    }

    /// Angle of the complex number.
    #[inline(always)]
    fn ang( &self ) -> T
    {
        return self.arg();
    }
}


impl <T: MixedNum + MixedNumSigned + MixedOps> core::ops::Mul<Complex<T>> for Complex<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        return ops::mul_cartesian(self, rhs);
    }
}

impl <T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedWrapPhase + MixedOps + ToComplex<T>> core::ops::Mul<Polar<T>> for Complex<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Polar<T>) -> Self {
        let rhs_cartesian = rhs.to_complex();
        return ops::mul_cartesian(self, rhs_cartesian);
    }
}

impl <T: MixedNum + MixedNumSigned + MixedOps> core::ops::Mul<T> for Complex<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self {
        return Complex::new(self.re*rhs, self.im*rhs);
    }
}

impl <T: MixedNum + MixedNumSigned + MixedOps + MixedZero> core::ops::Div<T> for Complex<T> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self {
        if rhs == T::mixed_zero() {
            return Complex::new(T::mixed_max_value(), T::mixed_max_value());
        }
        return Complex::new(self.re/rhs, self.im/rhs);
    }
}