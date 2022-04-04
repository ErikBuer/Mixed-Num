use crate::*;

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
    }
}

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

impl_mixed_num_for_primitive!(isize);

impl_mixed_num_for_primitive!(i8);
impl_mixed_num_for_primitive!(i16);
impl_mixed_num_for_primitive!(i32);
impl_mixed_num_for_primitive!(i64);
impl_mixed_num_for_primitive!(i128);