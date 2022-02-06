use crate::*;
use num::traits::float::FloatCore;

use libm;


macro_rules! impl_mixed_num_for_primitive{
    ( $T:ty ) => {

        impl MixedNumConversion<f32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:f32 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> f32 {
                return *self as f32;
            }
        }

        impl MixedNumConversion<f64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:f64 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> f64 {
                return *self as f64;
            }
        }

        impl MixedNumConversion<u32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:u32 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> u32 {
                return *self as u32;
            }
        }

        impl MixedNumConversion<u64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:u64 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> u64 {
                return *self as u64;
            }
        }

        impl MixedNumConversion<i32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:i32 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> i32 {
                return *self as i32;
            }
        }

        impl MixedNumConversion<i64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:i64 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> i64 {
                return *self as i64;
            }
        }

        impl MixedWrapPhase for $T
        {
            #[inline(always)]
            fn mixed_wrap_phase(&self) -> Self {
                return trigonometry::wrap_phase(*self);
            }
        }

        impl MixedOps for $T
        {
            #[inline(always)]
            fn mixed_abs( &self ) -> Self {
                return self.abs();
            }
            #[inline(always)]
            fn mixed_powi( &self, exp: i32 ) -> Self {
                return self.powi( exp );
            }
        }

        impl MixedNumSigned for $T
        {   
        }
        
        impl MixedNum for $T
        {
            #[inline(always)]
            fn mixed_max_value() -> Self {
                return Self::max_value();
            }
            #[inline(always)]
            fn mixed_min_value() -> Self {
                return Self::min_value();
            }
            fn mixed_sign( &self) -> Self {
                return trigonometry::sign(*self);
            }
            #[inline(always)]
            fn mixed_is_positive( &self) -> bool {
                return self.is_sign_positive();
            }
            #[inline(always)]
            fn mixed_is_negative( &self) -> bool {
                return self.is_sign_negative();
            }
        }

        impl MixedZero for $T
        {
            #[inline(always)]
            fn mixed_zero() -> Self {
                return 0 as $T;
            }
        }

        impl MixedOne for $T
        {
            #[inline(always)]
            fn mixed_one() -> Self {
                return 1 as $T;
            }
        }

        impl MixedPi for $T
        {
            #[inline(always)]
            fn mixed_pi() -> Self {
                return 3.1415926535897932384626433832795028841971693993751058209749445923078164062 as $T;
            }
            #[inline(always)]
            fn mixed_tau() -> Self {
                return 6.2831853071795864769252867665590057683943387987502116419498891846156328124 as $T;
            }
        }

        impl MixedConsts for $T
        {
        }
    }
}

// ________________________________________________________________________________________________________________________________________
// Implementations for f32


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

impl_mixed_num_for_primitive!(f32);


// ________________________________________________________________________________________________________________________________________
// Implementations for f64

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

impl_mixed_num_for_primitive!(f64);