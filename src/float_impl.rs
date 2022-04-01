use crate::*;
use num::traits::float::FloatCore;

use libm;

mod f32_impl;
pub use f32_impl::*;
mod f64_impl;
pub use f64_impl::*;


macro_rules! impl_mixed_num_for_primitive{
    ( $T:ty ) => {

        impl MixedNum for $T
        {
        }

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
        /*
        impl MixedNumConversion<Cartesian<f32>> for $T
        {
            /// Only uses the real part.
            #[inline(always)]
            fn mixed_from_num( number:Cartesian<f32> ) -> Self {
                return number.re as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> Cartesian<f32> {
                return Cartesian::new(*self as f32, f32::mixed_zero());
            }
        }

        impl MixedNumConversion<Cartesian<f64>> for $T
        {
            /// Only uses the real part.
            #[inline(always)]
            fn mixed_from_num( number:Cartesian<f64> ) -> Self {
                return number.re as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> Cartesian<f64> {
                return Cartesian::new(*self as f64, f64::mixed_zero());
            }
        }
        */

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

        impl MixedAbs for $T
        {
            #[inline(always)]
            fn mixed_abs( &self ) -> Self {
                return self.abs();
            }
        }

        impl MixedPowi for $T
        {
            #[inline(always)]
            fn mixed_powi( &self, exp: i32 ) -> Self {
                return self.powi( exp );
            }
        }
        
        impl MixedOps for $T
        {
        }

        impl MixedNumSigned for $T
        {   
        }
        
        impl MixedReal for $T
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

        impl DbMag for $T
        {
            fn mixed_mag2db(&self) -> Self
            {
                return <$T>::mixed_from_num(20)*self.mixed_log10();
            }
            fn mixed_db2mag(&self) -> Self
            {
                let exponent: $T = *self/<$T>::mixed_from_num(20i32);
                return exponent.mixed_exp10();
            }
        }

        impl DbPow for $T
        {
            fn mixed_pow2db(&self) -> Self
            {
                return <$T>::mixed_from_num(10)*self.mixed_log10();
            }
            fn mixed_db2pow(&self) -> Self
            {
                let exponent: $T = *self/<$T>::mixed_from_num(10i32);
                return exponent.mixed_exp10();
            }
        }
    }
}

impl_mixed_num_for_primitive!(f32);
impl_mixed_num_for_primitive!(f64);


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn pow2db() {
        let numb = 2f32;
        assert_eq!(numb.mixed_pow2db(), 3.0103002);
    }

    #[test]
    fn db2pow() {
        let numb = 10f32;
        assert_eq!(numb.mixed_db2pow(), 10.0);
    }

    #[test]
    fn mag2db() {
        let numb = 2f32;
        assert_eq!(numb.mixed_mag2db(), 6.0206003);
    }

    #[test]
    fn db2mag() {
        let numb = 10f32;
        assert_eq!(numb.mixed_db2mag(), 3.1622777);
    }
}