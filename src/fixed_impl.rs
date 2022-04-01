use crate::*;
use fixed;

macro_rules! impl_mixed_num_for_fixed{
    ( $T:ty ) => {

        impl MixedNum  for $T
        {
        }

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
        /*
        impl MixedNumConversion<Cartesian<$T>> for $T
        {
            /// Only uses the real part.
            #[inline(always)]
            fn mixed_from_num( number:Cartesian<$T> ) -> Self {
                return number.re;
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> Cartesian<$T> {
                return Cartesian::new(*self, <$T>::mixed_zero());
            }
        }

        impl MixedNumConversion<Cartesian<f32>> for $T
        {
            /// Only uses the real part.
            #[inline(always)]
            fn mixed_from_num( number:Cartesian<f32> ) -> Self {
                return Self::from_num(number.re);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> Cartesian<f32> {
                return Cartesian::new(self.to_num::<f32>(), f32::mixed_zero());
            }
        }

        impl MixedNumConversion<Cartesian<f64>> for $T
        {
            /// Only uses the real part.
            #[inline(always)]
            fn mixed_from_num( number:Cartesian<f64> ) -> Self {
                return Self::from_num(number.re);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> Cartesian<f64> {
                return Cartesian::new(self.to_num::<f64>(), f64::mixed_zero());
            }
        }
        */

        impl MixedNumConversion<u32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:u32 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> u32 {
                return self.to_num::<u32>();
            }
        }

        impl MixedNumConversion<u64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:u64 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> u64 {
                return self.to_num::<u64>();
            }
        }

        impl MixedNumConversion<i32> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:i32 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> i32 {
                return self.to_num::<i32>();
            }
        }

        impl MixedNumConversion<i64> for $T
        {
            #[inline(always)]
            fn mixed_from_num( number:i64 ) -> Self {
                return Self::from_num(number);
            }
            #[inline(always)]
            fn mixed_to_num( &self ) -> i64 {
                return self.to_num::<i64>();
            }
        }

        impl MixedPi for $T
        {
            #[inline(always)]
            fn mixed_pi() -> Self {
                return Self::from_num(3.1415926535897932384626433832795028841971693993751058209749445923078164062);
            }
            #[inline(always)]
            fn mixed_tau() -> Self {
                return Self::from_num(6.2831853071795864769252867665590057683943387987502116419498891846156328124);
            }
        }

        impl MixedZero for $T
        {
            /// Return the zero value of type Self.
            #[inline(always)]
            fn mixed_zero() -> Self {
                return Self::from_num(0) as $T;
            }
        }

        impl MixedOne for $T
        {
            /// Return the zero value of type Self.
            #[inline(always)]
            fn mixed_one() -> Self {
                return Self::from_num(1) as $T;
            }
        }

        impl MixedConsts for $T
        {
        }

        impl MixedSqrt for $T
        {
            /// Take the square root of self.
            #[inline(always)]
            fn mixed_sqrt(&self) -> Self {
                return trigonometry::sqrt::niirf(*self, 2);
            }
            /// Take the square root of self.
            #[inline(always)]
            fn mixed_niirf(&self) -> Self {
                return trigonometry::sqrt::niirf(*self, 2);
            }
        }
    }
}

macro_rules! impl_mixed_num_for_fixed_unsigned{
    ( $T:ty ) => {
        impl_mixed_num_for_fixed!($T);

        impl MixedOps for $T
        {
        }

        impl MixedAbs for $T
        {
            #[inline(always)]
            fn mixed_abs( &self ) -> Self {
                return *self; // Is itself for unsigned.
            }
        }

        impl MixedPowi for $T
        {
            #[inline(always)]
            fn mixed_powi( &self, exp: i32 ) -> Self {
                return trigonometry::powi( *self, exp as usize );
            }
        }

        impl MixedReal for $T
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
            fn mixed_sign( &self) -> Self {
                return trigonometry::sign(*self);
            }
            #[inline(always)]
            fn mixed_is_positive( &self) -> bool {
                return true;
            }
            #[inline(always)]
            fn mixed_is_negative( &self) -> bool {
                return false;
            }
        }
    }
}

macro_rules! impl_mixed_num_for_fixed_signed{
    ( $T:ty ) => {
        impl_mixed_num_for_fixed!($T);

        impl MixedNumSigned  for $T
        {
        }

        impl MixedOps for $T
        {
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
                return trigonometry::powi( *self, exp as usize );
            }
        }

        impl MixedReal for $T
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
            fn mixed_sign( &self) -> Self {
                return trigonometry::sign(*self);
            }
            #[inline(always)]
            fn mixed_is_positive( &self) -> bool {
                return self.is_positive();
            }
            #[inline(always)]
            fn mixed_is_negative( &self) -> bool {
                return self.is_negative();
            }
        }

        impl MixedWrapPhase for $T
        {
            #[inline(always)]
            fn mixed_wrap_phase(&self) -> Self {
                return trigonometry::wrap_phase(*self);
            }
        }

        impl MixedSin for $T
        {
            /// Take the sin of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_sin(&self) -> Self {
                return cordic::sin(*self);
            }
            #[inline(always)]
            fn mixed_sincos(&self) -> (Self, Self) 
                where Self: Sized
            {
                return cordic::sin_cos(*self);
            }
            /// Take the arcsin of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_asin(&self) -> Self {
                return cordic::asin(*self);
            }
        }

        impl MixedCos for $T
        {
            /// Take the cos of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_cos(&self) -> Self {
                return cordic::cos(*self);
            }
            /// Take the arccos of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_acos(&self) -> Self {
                return cordic::acos(*self);
            }
        }

        impl MixedTrigonometry for $T
        {    
        }

        impl MixedAtan for $T
        {
            /// Take the atan of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_atan(&self) -> Self {
                return cordic::atan(*self);
            }
            /// Take the atan of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_atan2(&self, other:Self) -> Self {
                return cordic::atan2(*self, other);
            }
            /// Take the atan of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_atan2_poly(&self, other:Self) -> Self {
                return trigonometry::atan::atan2(*self, other);
            }
        }

        impl MixedExp for $T
        {
            /// Take the exp() of self. Implementation varies with type.
            #[inline(always)]
            fn mixed_exp(&self) -> Self {
                return cordic::exp(*self);
            }
        }
    }
}

impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U5>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U6>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U7>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU8<fixed::types::extra::U8>);

impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U5>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U6>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U7>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI8<fixed::types::extra::U8>);


impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U13>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U14>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U15>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU16<fixed::types::extra::U16>);

impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U13>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U14>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U15>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI16<fixed::types::extra::U16>);


impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U16>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U17>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U18>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U19>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U20>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U21>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U22>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U23>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U24>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U25>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U26>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U27>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U28>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U29>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U30>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U31>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU32<fixed::types::extra::U32>);

impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U16>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U17>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U18>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U19>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U20>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U21>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U22>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U23>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U24>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U25>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U26>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U27>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U28>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U29>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U30>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U31>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI32<fixed::types::extra::U32>);


impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U16>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U17>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U18>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U19>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U20>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U21>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U22>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U23>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U24>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U25>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U26>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U27>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U28>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U29>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U30>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U31>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U32>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U33>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U34>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U35>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U36>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U37>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U38>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U39>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U40>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U41>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U42>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U43>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U44>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U45>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U46>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U47>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U48>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U49>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U50>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U51>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U52>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U53>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U54>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U55>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U56>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U57>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U58>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U59>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U60>);
impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U61>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U62>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U63>);
//impl_mixed_num_for_fixed_unsigned!(fixed::FixedU64<fixed::types::extra::U64>);

impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U0>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U1>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U2>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U3>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U4>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U5>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U6>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U7>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U8>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U9>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U10>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U11>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U12>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U13>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U14>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U15>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U16>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U17>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U18>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U19>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U20>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U21>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U22>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U23>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U24>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U25>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U26>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U27>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U28>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U29>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U30>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U31>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U32>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U33>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U34>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U35>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U36>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U37>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U38>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U39>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U40>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U41>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U42>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U43>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U44>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U45>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U46>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U47>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U48>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U49>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U50>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U51>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U52>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U53>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U54>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U55>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U56>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U57>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U58>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U59>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U60>);
impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U61>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U62>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U63>);
//impl_mixed_num_for_fixed_signed!(fixed::FixedI64<fixed::types::extra::U64>);
