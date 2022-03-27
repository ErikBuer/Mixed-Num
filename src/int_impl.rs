use crate::*;

macro_rules! impl_mixed_num_for_primitive{
    ( $T:ty ) => {

        impl MixedNum for $T
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
                return self.is_positive()
            }
            #[inline(always)]
            fn mixed_is_negative( &self) -> bool {
                return self.is_negative();
            }
        }

        impl MixedOps for $T
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

        impl MixedAbs for $T
        {
            #[inline(always)]
            fn mixed_abs( &self ) -> Self {
                if 0<=*self
                {
                    return *self;
                }
                else
                {
                    return -self;
                }
            }
        }

        impl MixedPowi for $T
        {
            #[inline(always)]
            fn mixed_powi( &self, exp: i32 ) -> Self {
                return trigonometry::powi( *self, exp as usize );
            }
        }

        impl MixedNumSigned for $T
        {   
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
    }
}

impl_mixed_num_for_primitive!(i8);
impl_mixed_num_for_primitive!(i16);
impl_mixed_num_for_primitive!(i32);
impl_mixed_num_for_primitive!(i64);
