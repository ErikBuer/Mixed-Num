use super::*;

/// Cast cartesian complex fixed point number to polar form.
/// 
/// ## Arguments
/// 
/// * `x` - The number transform.
///
pub fn to_polar<T>( x: Complex<T> ) -> Polar<T>
    where T:  MixedNum + MixedNumSigned + MixedSqrt + MixedAtan + MixedPowi + MixedAbs + MixedOps
{
    let c_polar = Polar::<T>{
        mag:     abs(x),
        ang: x.im.mixed_atan2(x.re)
    };
    return c_polar;
}

/// Calculate the absolute value of the argument.
/// 
/// ## Arguments
/// 
/// * `a` - The argument to apply the function to.
/// 
/// ## Example
/// 
/// ```
/// use Complex;
/// use fixed_trigonometry::complex::*;
/// 
/// let mut x = Complex{re:1f32, im:0f32};
/// assert_eq!{ abs(x), 1f32 };
/// ``` 
/// 
pub fn abs<T>( a: Complex<T> ) -> T
where T: MixedNum + MixedNumSigned + MixedSqrt + MixedPowi + MixedOps
{
    let r_sqr = a.re.mixed_powi(2) + a.im.mixed_powi(2);
    return r_sqr.mixed_sqrt();
}

/// Cast cartesian complex fixed point number to polar form.
/// 
/// ## Arguments
/// 
/// * `a` - The number transform.
/// 
/// ## Example
/// 
/// ```
/// use Complex;
/// use fixed_trigonometry::complex::*;
/// 
/// let mut x = Polar{r:1f32, theta:0f32};
/// assert_eq!{ to_cartsian(x).to_string(), "1+0i" };
/// ``` 
/// 
pub fn to_cartsian<T>( a: Polar<T> ) -> Complex<T>
    where T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedWrapPhase + MixedOps
{
    let theta = a.ang.mixed_wrap_phase();
    let (imag_s, real_s) = theta.mixed_sincos();

    let c_cartesian = Complex::<T>{
        re: a.mag*real_s,
        im: a.mag*imag_s
    };
    return c_cartesian;
}

/// Add two complex fixed-point numbers in cartesian form.
/// 
pub fn add<T>( a: Complex<T>, b: Complex<T> ) -> Complex<T>
    where T: MixedNum + MixedOps
{
    let c_cartesian = Complex::<T>{
        re: a.re + b.re,
        im: a.im + b.im
    };
    return c_cartesian;
}

/// Subtract b from a.
/// c = a-b
/// 
pub fn sub<T>( a: Complex<T>, b: Complex<T> ) -> Complex<T>
    where T: MixedNum + MixedNumSigned +  core::ops::Sub<Output = T>
{
    let c_cartesian = Complex::<T>{
        re: a.re - b.re,
        im: a.im - b.im
    };
    return c_cartesian;
}

/// Multiply fixed-point complex numbers in polar form.
/// 
pub fn mul_polar<T>( a: Polar<T>, b: Polar<T> ) -> Polar<T>
    where T: MixedNum + MixedNumSigned + MixedOps
{
    if a.mag==T::mixed_from_num(0) || b.mag==T::mixed_from_num(0)
    {
        let c = Polar::<T>{
            mag:     T::mixed_from_num(0),
            ang: T::mixed_from_num(0)
        };
        return c;
    }
    else
    {
        let c = Polar::<T>{
            mag: a.mag*b.mag,
            ang: a.ang+b.ang
        };
        return c;
    }
}

/// Multiply two cartesian complex numbers.
/// 
pub fn mul_cartesian<T>( ab: Complex<T>, bc: Complex<T> ) -> Complex<T>
    where T:  MixedNum + MixedNumSigned + MixedOps 
{   
    let a = ab.re;
    let b = ab.im;
    let c = bc.re;
    let d = bc.im;

    let re = (a*c) - (b*d);
    let im = (a*d) + (b*c);
    return Complex{re:re, im:im}
}

/// Rase a complex fixed-point number to an real-valued integer power.
/// `base^power`.
/// 
/// ## Arguments
/// 
/// * `base`  - The complex, fixed-point base number.
/// * `power` - The power to raise 'base' to.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry as trig;
/// use fixed::{types::extra::U22, FixedI32};
/// use Complex;
/// 
/// 
/// let x = Complex::new( FixedI32::<U22>::from_num(1), FixedI32::<U22>::from_num(1) );
/// let y = trig::complex::powi( x, 2 );
/// 
/// let result = Complex::new( FixedI32::<U22>::from_num( -0.0000038, ), FixedI32::<U22>::from_num( 1.996035 ));
/// assert_eq!{ y, result };
/// ```
/// 
pub fn powi<T>( base: Complex<T>, power:usize ) -> Complex<T>
    where T: fixed::traits::FixedSigned + MixedNum + MixedNumSigned + MixedAtan + MixedSin + MixedSqrt + MixedPowi
{   
    // Calculate raised magnitude.
    let temp:T = base.re.mixed_powi(2) + base.im.mixed_powi(2);
    let mag:T  = temp.mixed_sqrt().mixed_powi(power as i32);

    let phi:T  = base.im.mixed_atan2(base.re) *<T>::from_num(power);

    let (imag_s, real_s) = phi.mixed_sincos();

    let real   = mag*real_s;
    let imag   = mag*imag_s;

    return Complex::new_from_cartesian( real, imag);
}

/// Divide a cartesian complex by a real scalar.
/// c = a/b
/// 
pub fn div_cartesian<T>( a: Complex<T>, b: T  ) -> Complex<T>
    where T: MixedNum + MixedNumSigned + MixedOps
{
    let mut c = a;

    if b == T::mixed_from_num(0)
    {
        c.re = T::mixed_max_value();
        c.im = T::mixed_max_value();
    }
    else
    {
        c.re = a.re / b;
        c.im = a.im / b;
    }
    return c;
}