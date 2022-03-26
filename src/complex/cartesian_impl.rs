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

impl <T: MixedNum + MixedNumSigned + MixedWrapPhase + MixedTrigonometry> NewFromPolar<T> for Complex<T>
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

impl <T: MixedNum + MixedZero> MixedZero for Complex<T>
{
    /// Return the zero value of type Self.
    #[inline(always)]
    fn mixed_zero() -> Self {
        return Self{re:T::mixed_zero(), im:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedZero> MixedOne for Complex<T>
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