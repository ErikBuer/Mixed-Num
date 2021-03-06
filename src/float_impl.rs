use crate::*;
use num::traits::float::FloatCore;

use libm;

mod f32_impl;
pub use f32_impl::*;
mod f64_impl;
pub use f64_impl::*;

macro_rules! impl_mixed_num_conversion{
    ( $T1:ty, $T2:ty ) => {
        impl MixedNumConversion<$T2> for $T1
        {
            #[inline(always)]
            fn mixed_from_num( number:$T2 ) -> Self {
                return number as Self;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> $T2 {
                return *self as $T2;
            }
        }

        impl MixedNumConversion<Cartesian<$T2>> for $T1
        {
            /// Extracts real part of self, including type cast to the target type.
            /// 
            /// ## Example
            /// 
            /// ```
            /// use mixed_num::*;
            /// use mixed_num::traits::*;
            ///
            /// let num = Cartesian::new(1f32,0f32);
            /// 
            /// let real_num:f64 = f64::mixed_from_num(num);
            /// 
            /// assert_eq!{ real_num, 1f64 };
            /// ```
            #[inline(always)]
            fn mixed_from_num( number:Cartesian<$T2> ) -> Self {
                return number.re as Self;
            }
            /// Casting real number to a complex, including type cast of T1 to T2 (Cartesian<T2>). 
            ///  
            /// ## Example
            /// 
            /// ```
            /// use mixed_num::*;
            /// use mixed_num::traits::*;
            /// 
            /// // Notice the support cor type cast as well as real/complex conversion. 
            /// let num: Cartesian<f64> = 2f32.mixed_to_num();
            /// 
            /// assert_eq!{ num.to_string(), "2+0i" };
            /// ```
            #[inline(always)]
            fn mixed_to_num( &self ) -> Cartesian<$T2> {
                return Cartesian::new(*self as $T2, <$T2>::mixed_zero());
            }
        }
    }
}

macro_rules! impl_mixed_num_for_primitive{
    ( $T:ty ) => {

        impl MixedNum for $T
        {
        }

        impl_mixed_num_conversion!($T, f32);
        impl_mixed_num_conversion!($T, f64);
        
        impl_mixed_num_conversion!($T, usize);
        impl_mixed_num_conversion!($T, isize);
        
        impl_mixed_num_conversion!($T, u8);
        impl_mixed_num_conversion!($T, u16);
        impl_mixed_num_conversion!($T, u32);
        impl_mixed_num_conversion!($T, u64);
        impl_mixed_num_conversion!($T, u128);

        impl_mixed_num_conversion!($T, i8);
        impl_mixed_num_conversion!($T, i16);
        impl_mixed_num_conversion!($T, i32);
        impl_mixed_num_conversion!($T, i64);
        impl_mixed_num_conversion!($T, i128);

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