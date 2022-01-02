//! 
//! No-STD abstraction layer enabling numerical functions to be implemented once, and simultaneously support both fixed and floating point types.
//! 

#![crate_name = "mixed_num"]
#![no_std]

use fixed;
use num::traits::float::FloatCore;
use fixed_trigonometry as trig;

pub trait MixedNumConversion<T> {
    /// Generic type cast from numeric type T.
    fn mixed_from_num( number:T ) -> Self;
    /// Generic type cast to  numeric type T.
    fn mixed_to_num( &self )      -> T;
}

pub trait MixedNum
{
    /// Maximum value of the type.
    fn mixed_max_value() -> Self;
    /// Minimum value of the type.
    fn mixed_min_value() -> Self;
    /// Absolute value.
    fn mixed_abs( &self ) -> Self;
    /// Integer valued power.
    fn mixed_powi( &self, exp: i32 ) -> Self;
}


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
            #[inline(always)]
            fn mixed_abs( &self ) -> Self {
                return self.abs();
            }
            #[inline(always)]
            fn mixed_powi( &self, exp: i32 ) -> Self {
                return self.powi( exp );
            }
        }
    }
}

impl_mixed_num_for_primitive!(f32);
impl_mixed_num_for_primitive!(f64);


macro_rules! impl_mixed_num_for_fixed{
    ( $T:ty ) => {

        impl MixedNumConversion<f32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:f32 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> f32 {
                return self.to_num::<f32>();
            }
        }

        impl MixedNumConversion<f64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:f64 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> f64 {
                return self.to_num::<f64>();
            }
        }

        impl MixedNum for $T
        {
            #[inline(always)]
            fn mixed_max_value() -> Self {
                return Self::MAX;
            }
            #[inline(always)]
            fn mixed_min_value() -> Self {
                return Self::MIN;
            }
            #[inline(always)]
            fn mixed_abs( &self ) -> Self {
                return self.abs();
            }
            #[inline(always)]
            fn mixed_powi( &self, exp: i32 ) -> Self {
                return trig::powi( *self, exp as usize );
            }
        }
    }
}

/*
impl_mixed_num_for_fixed!(fixed::FixedU8<fixed::types::extra::U0>);
impl_mixed_num_for_fixed!(fixed::FixedU8<fixed::types::extra::U1>);
impl_mixed_num_for_fixed!(fixed::FixedU8<fixed::types::extra::U2>);
impl_mixed_num_for_fixed!(fixed::FixedU8<fixed::types::extra::U3>);
impl_mixed_num_for_fixed!(fixed::FixedU8<fixed::types::extra::U4>);
impl_mixed_num_for_fixed!(fixed::FixedU8<fixed::types::extra::U5>);
impl_mixed_num_for_fixed!(fixed::FixedU8<fixed::types::extra::U6>);
impl_mixed_num_for_fixed!(fixed::FixedU8<fixed::types::extra::U7>);
impl_mixed_num_for_fixed!(fixed::FixedU8<fixed::types::extra::U8>);
*/
impl_mixed_num_for_fixed!(fixed::FixedI8<fixed::types::extra::U0>);
impl_mixed_num_for_fixed!(fixed::FixedI8<fixed::types::extra::U1>);
impl_mixed_num_for_fixed!(fixed::FixedI8<fixed::types::extra::U2>);
impl_mixed_num_for_fixed!(fixed::FixedI8<fixed::types::extra::U3>);
impl_mixed_num_for_fixed!(fixed::FixedI8<fixed::types::extra::U4>);
impl_mixed_num_for_fixed!(fixed::FixedI8<fixed::types::extra::U5>);
impl_mixed_num_for_fixed!(fixed::FixedI8<fixed::types::extra::U6>);
impl_mixed_num_for_fixed!(fixed::FixedI8<fixed::types::extra::U7>);
impl_mixed_num_for_fixed!(fixed::FixedI8<fixed::types::extra::U8>);

/*
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U0>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U1>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U2>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U3>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U4>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U5>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U6>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U7>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U8>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U9>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U10>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U11>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U12>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U13>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U14>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U15>);
impl_mixed_num_for_fixed!(fixed::FixedU16<fixed::types::extra::U16>);
 */
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U0>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U1>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U2>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U3>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U4>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U5>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U6>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U7>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U8>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U9>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U10>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U11>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U12>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U13>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U14>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U15>);
impl_mixed_num_for_fixed!(fixed::FixedI16<fixed::types::extra::U16>);

/*
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U0>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U1>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U2>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U3>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U4>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U5>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U6>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U7>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U8>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U9>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U10>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U11>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U12>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U13>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U14>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U15>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U16>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U17>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U18>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U19>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U20>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U21>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U22>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U23>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U24>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U25>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U26>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U27>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U28>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U29>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U30>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U31>);
impl_mixed_num_for_fixed!(fixed::FixedU32<fixed::types::extra::U32>);
*/
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U0>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U1>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U2>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U3>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U4>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U5>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U6>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U7>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U8>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U9>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U10>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U11>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U12>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U13>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U14>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U15>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U16>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U17>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U18>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U19>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U20>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U21>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U22>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U23>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U24>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U25>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U26>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U27>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U28>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U29>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U30>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U31>);
impl_mixed_num_for_fixed!(fixed::FixedI32<fixed::types::extra::U32>);

/*
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U0>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U1>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U2>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U3>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U4>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U5>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U6>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U7>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U8>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U9>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U10>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U11>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U12>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U13>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U14>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U15>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U16>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U17>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U18>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U19>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U20>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U21>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U22>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U23>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U24>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U25>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U26>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U27>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U28>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U29>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U30>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U31>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U32>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U33>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U34>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U35>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U36>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U37>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U38>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U39>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U40>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U41>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U42>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U43>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U44>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U45>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U46>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U47>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U48>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U49>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U50>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U51>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U52>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U53>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U54>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U55>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U56>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U57>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U58>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U59>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U60>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U61>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U62>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U63>);
impl_mixed_num_for_fixed!(fixed::FixedU64<fixed::types::extra::U64>);
 */
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U0>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U1>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U2>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U3>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U4>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U5>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U6>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U7>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U8>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U9>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U10>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U11>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U12>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U13>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U14>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U15>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U16>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U17>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U18>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U19>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U20>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U21>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U22>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U23>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U24>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U25>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U26>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U27>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U28>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U29>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U30>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U31>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U32>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U33>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U34>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U35>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U36>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U37>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U38>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U39>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U40>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U41>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U42>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U43>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U44>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U45>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U46>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U47>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U48>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U49>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U50>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U51>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U52>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U53>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U54>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U55>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U56>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U57>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U58>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U59>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U60>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U61>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U62>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U63>);
impl_mixed_num_for_fixed!(fixed::FixedI64<fixed::types::extra::U64>);