use crate::*;

pub trait ToCartesian<T> {
    fn to_cartesian( &self ) -> Cartesian<T>;
}

pub trait ToPolar<T> {
    fn to_polar( &self ) -> Polar<T>;
}

pub trait MixedComplexConversion<T> {
    /// Type cast from real number T to Complex<T>.
    fn mixed_to_complex( number:T ) -> Self;
}

pub trait NewFromCartesian<T> 
    where Self: MixedComplex
{
    /// Create a complex number from cartesian coordinates.
    fn new_from_cartesian( re:T, im:T ) -> Self;
}

pub trait NewFromPolar<T> 
    where Self: MixedComplex
{
    /// Create a complex number from polar coordinates.
    fn new_from_polar( mag:T, ang:T ) -> Self;
}

pub trait Mag<T>
    where Self: MixedComplex
{
    /// Magnitude of the complex number.
    fn mag( &self ) -> T;
    /// Magnitude of the complex number.
    fn abs( &self ) -> T;
}

pub trait Arg<T> 
    where Self: MixedComplex
{
    /// Argument of the complex number.
    fn arg( &self ) -> T;

    /// Angle of the complex number.
    fn ang( &self ) -> T;
}