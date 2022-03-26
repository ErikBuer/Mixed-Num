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

impl <T: MixedNum + MixedNumSigned + MixedSqrt + MixedAtan> NewFromCartesian<T> for Polar<T>
{
    /// Type cast from real number T to Complex<T>.
    fn new_from_cartesian( re:T, im:T ) -> Self
    {
        return ops::to_polar(Complex::new(re,im));
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

impl <T: MixedNum + MixedZero> MixedZero for Polar<T>
{
    /// Return the zero value of type Self.
    #[inline(always)]
    fn mixed_zero() -> Self {
        return Self{mag:T::mixed_zero(), ang:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedZero> MixedOne for Polar<T>
{
    /// Return the zero value of type Self.
    #[inline(always)]
    fn mixed_one() -> Self {
        return Self{mag:T::mixed_one(), ang:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedZero> MixedComplexConversion<T>  for Polar<T>
{
    /// Type cast from real number T to Complex<T>.
    fn mixed_to_complex( number:T ) -> Self {
        return Self{mag:number, ang:T::mixed_zero()};
    }
}