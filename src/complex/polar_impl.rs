use super::*;

impl <T: MixedNum + MixedNumSigned> MixedComplex for Polar<T>
{
}

impl<T> Polar<T>
{
    pub fn new(mag:T, ang:T) -> Self
    {
        return Polar{mag:mag, ang:ang};
    }
}

impl <T: MixedNum + MixedNumSigned + MixedSqrt + MixedAtan + MixedPowi + MixedAbs + MixedOps > NewFromCartesian<T> for Polar<T>
{
    /// Type cast from real number T to Cartesian<T>.
    fn new_from_cartesian( re:T, im:T ) -> Self
    {
        return ops::to_polar(Cartesian::new(re,im));
    }
}

impl <T: MixedNum + MixedNumSigned + MixedWrapPhase + MixedTrigonometry> NewFromPolar<T> for Polar<T>
{
    fn new_from_polar( mag:T, ang:T ) -> Self
    {
        return Polar::new(mag, ang);
    }
}

impl <T: MixedNum + MixedNumConversion<T2> + MixedZero, T2: MixedNum> MixedNumConversion<T2> for Polar<T>
{
    #[inline(always)]
    fn mixed_from_num( number:T2 ) -> Self {
        return Self{mag:T::mixed_from_num(number), ang:T::mixed_zero()};
    }
    #[inline(always)]
    fn mixed_to_num( &self ) -> T2 {
        return self.mag.mixed_to_num();
    }
}

impl <T: MixedNum + MixedNumSigned + MixedWrapPhase + MixedOps + MixedTrigonometry> ToCartesian<T> for Polar<T>
{
    /// Cartesian<T> to Polar<T>.
    #[inline(always)]
    fn to_cartesian( &self ) -> Cartesian<T>
    {
        return ops::to_cartesian(*self);
    }
}

impl <T: MixedNum + MixedNumSigned> ToPolar<T> for Polar<T>
{
    /// Cartesian<T> to Polar<T>.
    #[inline(always)]
    fn to_polar( &self ) -> Polar<T>
    {
        return *self;
    }
}

impl <T: MixedNum + MixedZero> MixedZero for Polar<T>
{
    /// Return the zero value of type Self.
    #[inline(always)]
    fn mixed_zero() -> Self {
        return Self{mag:T::mixed_zero(), ang:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedZero + MixedOne> MixedOne for Polar<T>
{
    /// Return the zero value of type Self.
    #[inline(always)]
    fn mixed_one() -> Self {
        return Self{mag:T::mixed_one(), ang:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedZero> MixedComplexConversion<T>  for Polar<T>
{
    /// Type cast from real number T to Polar<T>.
    fn mixed_to_complex( number:T ) -> Self {
        return Self{mag:number, ang:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedNumSigned> Mag<T> for Polar<T>
{
    /// Magnitude of the complex number.
    #[inline(always)]
    fn mag( &self ) -> T
    {
        return self.mag;
    }
    /// Magnitude of the complex number.
    #[inline(always)]
    fn abs( &self ) -> T
    {
        return self.mag;
    }
}

impl <T: MixedNum + MixedNumSigned + MixedZero> MixedAbs for Polar<T>
{
    #[inline(always)]
    fn mixed_abs( &self ) -> Self
    {
        return Self::new(self.mag, T::mixed_zero());
    }
}

impl <T: MixedNum + MixedNumSigned> Arg<T> for Polar<T>
{
    /// Argument of the complex number.
    #[inline(always)]
    fn arg( &self ) -> T
    {
        return self.ang;
    }

    /// Angle of the complex number.
    #[inline(always)]
    fn ang( &self ) -> T
    {
        return self.ang;
    }
}

impl <T: MixedNum + MixedNumSigned + MixedOps> core::ops::Mul<Polar<T>> for Polar<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        return ops::mul_polar(self, rhs);
    }
}

impl <T: MixedNum + MixedNumSigned + MixedSqrt + MixedOps + MixedAbs + MixedPowi + MixedAtan + ToPolar<T>> core::ops::Mul<Cartesian<T>> for Polar<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Cartesian<T>) -> Self {
        let rhs_pol = rhs.to_polar();
        return ops::mul_polar(self, rhs_pol);
    }
}

impl <T: MixedNum + MixedNumSigned + MixedOps> core::ops::Mul<T> for Polar<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self {
        return Polar::new(self.mag*rhs, self.ang);
    }
}

impl <T: MixedNum + MixedNumSigned + MixedOps + MixedZero> core::ops::Div<T> for Polar<T> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self {
        if rhs == T::mixed_zero() {
            return Polar::new(T::mixed_max_value(), self.ang);
        }
        return Polar::new(self.mag/rhs, self.ang);
    }
}

impl<T> core::fmt::Display for Polar<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}∠{}", self.mag, self.ang)
    }
}