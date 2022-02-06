pub mod atan;
pub mod sqrt;

/// Get the sign of the argument with a unit value.
/// Zero is of positive sign.
/// 
/// ## Arguments
/// 
/// * `x`  - The function argument.
/// 
/// ## Example
/// 
/// ```
/// use mixed_num::trigonometry::*;
/// use fixed::{types::extra::U22, FixedI32};
/// 
/// let mut x = FixedI32::<U22>::from_num(-0.2);
/// let mut y = sign(x);
/// assert_eq!{ y.to_num::<f32>(), -1.0 };
/// 
/// x = FixedI32::<U22>::from_num(0.2);
/// y = sign(x); 
/// assert_eq!{ y.to_num::<f32>(), 1.0 };
/// ``` 
pub fn sign<T>( x:T ) -> T
    where T: crate::MixedNum
{
    if x < T::mixed_from_num(0)
    {
        return T::mixed_from_num(-1);
    }
    else
    {
        return T::mixed_from_num(1);
    }
}

/// Calculate sin(x) using a Taylor approximation of `sin(x)`.
/// 
/// Sin is calculated using the following polynomial:
/// 
/// `sin(x) = x -( x^3/6 )+( x^5/120 )-( x^7/5040 )+( x^9/362880 )`
/// 
/// ## Argument
/// 
/// * `x` - The value to apply the operation to.
/// 
/// `x` must be wrapped to the -π=<x<π range.
/// 
/// ## Example
/// 
/// ```
/// use mixed_num::trigonometry::*;
/// 
/// let mut x:f32 = 0.0;
/// let mut y = sin(x);
/// assert_eq!{ y, 0.0 };
/// 
/// x = 3.1415/2.0;
/// y = sin(x);
/// assert_eq!{ y, 1.0000035 };
/// 
/// x = 3.1415;
/// y = sin(x);
/// assert_eq!{ y, 9.274483e-5 };
/// ``` 
/// 
#[allow(dead_code)]
pub fn sin<T>( x: T ) -> T
    where T: crate::MixedNum + crate::MixedNumSigned + crate::MixedPi
{
    let mixed_pi_half = T::mixed_pi()/T::mixed_from_num(2);

    let mut x_ = x;

    // Ensure that the angle is within the accurate range of the tailor series. 
    if x < -mixed_pi_half
    {   
        let delta:T = x+mixed_pi_half;
        x_ = -mixed_pi_half-delta;
    }
    else if mixed_pi_half < x
    {
        let delta:T = x-mixed_pi_half;
        x_ = mixed_pi_half-delta;
    }

    // Calculate sine by using 
    let mut sinx = x_-( x_.mixed_powi(3)/T::mixed_from_num(6) );
    sinx += x_.mixed_powi(5)/T::mixed_from_num(120);
    sinx -= x_.mixed_powi(7)/T::mixed_from_num(5040);
    sinx += x_.mixed_powi(9)/T::mixed_from_num(362880);
    return sinx;
}

/// Calculate cosine using a Taylor approximation of `cos(x)`.
/// 
/// Cos is calculated by adding a phase shift to x and running it through the polynomial sine method.
/// 
/// ## Argument
/// 
/// * `x` - The value to apply the operation to.
/// 
/// `x` is wrapped to the -π=<x<π range in the function.
/// 
/// ## Example
/// 
/// ```
/// use mixed_num::trigonometry::*;
/// 
/// let mut x = 0f32;
/// let mut y = cos(x);
/// assert_eq!{ y, 1.0000035 };
/// 
/// x = 3.1415f32/2.0f32;
/// y = cos(x);
/// assert_eq!{ y, 4.6491623e-5 };
/// ``` 
/// 
/// ## Comparison and Error
/// 
/// The figure below shows the comparison between the polynomial cosine, and the `std::f32::cos` implementation.
/// The Difference between the two is plotted as the error.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/polynomial_cosine_comparison.png?raw=true)
/// 
/// The error of the method is compared to the sine implementation in the cordic crate.
/// 
/// The comparison is done for U22 signed fixed point.
/// 
/// The figure below is missing numbers on the y axis, but it is plotted on a linear scale, showing the relative error between the two methods.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/cordic_poly_cos_error_comparison.png?raw=true)
/// 
#[allow(dead_code)]
pub fn cos<T>( x: T ) -> T
    where T: crate::MixedNum + crate::MixedNumSigned
{
    // shift to enable use of more accurate sinepolynomial method.
    let mixed_pi_half = T::mixed_pi()/T::mixed_from_num(2);

    let mut x_shifted = x+mixed_pi_half;
    x_shifted = wrap_phase(x_shifted);
    return sin(x_shifted);
}

/// Wrapps θ to the -π=<x<π range.
/// 
/// ## Arguments 
///
/// * `phi` - The unwrapped phase in radians.
/// 
/// ## Example
/// 
/// ```
/// use mixed_num::trigonometry::*;
/// use fixed::{types::extra::U28, FixedI32};
/// 
/// let phi =  FixedI32::<U28>::from_num(6);
/// let wrapped_phi = wrap_phase(phi);
/// assert_eq!{ wrapped_phi.to_num::<f32>(), -0.2831853 };
/// ``` 
pub fn wrap_phase<T>( phi: T ) -> T 
    where T: crate::MixedNum + crate::MixedNumSigned
{
    let mixed_pi  = T::mixed_pi();
    let tau = T::mixed_from_num(2)*mixed_pi;
 
    let mut temp_scalar = phi;
    
    while temp_scalar < -mixed_pi
    {
        temp_scalar = temp_scalar + tau;
    }
    while mixed_pi <= temp_scalar
    {
        temp_scalar = temp_scalar - tau;
    }
    return temp_scalar;
}

/// Rase fixed number to an integer-valued power.
/// `base^power`.
/// 
/// ## Arguments
/// 
/// * `base`  - The base number.
/// * `power` - The power to raise 'base' to.
/// 
/// ## Example
/// 
/// ```
/// use mixed_num::trigonometry::*;
/// use fixed::{types::extra::U22, FixedI32};
/// 
/// let mut x = FixedI32::<U22>::from_num(-2);
/// let y = powi(x, 2);
/// assert_eq!{ y.to_num::<f32>(), 4.0 };
/// ``` 
pub fn powi<T>( base:T, power:usize ) -> T
    where T: crate::MixedNum
{
    let mut temp:T = base;
    for _i in 0..power-1 {
        temp = temp*base;
    }
    return temp;
}