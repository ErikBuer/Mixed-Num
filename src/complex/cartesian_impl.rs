use super::*;

#[macro_use]
mod num_complex;

impl <T: MixedNum + MixedNumSigned> MixedComplex for Cartesian<T>
{
}

impl<T> Cartesian<T>
{
    pub fn new(re:T, im:T) -> Self
    {
        return Cartesian{re:re, im:im};
    }
}

impl <T: MixedNum + MixedNumSigned> NewFromCartesian<T> for Cartesian<T>
{
    /// Type cast from real number T to Complex<T>.
    fn new_from_cartesian( re:T, im:T ) -> Self
    {
        return Self{re:re, im:im};
    }
}

impl <T: MixedNum + MixedNumSigned + MixedWrapPhase + MixedTrigonometry + MixedOps> NewFromPolar<T> for Cartesian<T>
{
    fn new_from_polar( mag:T, ang:T ) -> Self
    {
        return ops::to_cartesian(Polar::new(mag, ang));
    }
}

impl <T: MixedNum + MixedNumConversion<T2> + MixedZero, T2: MixedNum> MixedNumConversion<T2> for Cartesian<T>
{
    #[inline(always)]
    fn mixed_from_num( number:T2 ) -> Self {
        return Self{re:T::mixed_from_num(number), im:T::mixed_zero()};
    }
    #[inline(always)]
    fn mixed_to_num( &self ) -> T2 {
        return self.re.mixed_to_num();
    }
}

impl <T: MixedNum + MixedNumSigned> ToCartesian<T> for Cartesian<T>
{
    /// Converison to Complex<T>.
    #[inline(always)]
    fn to_cartesian( &self ) -> Cartesian<T>
    {
        return *self;
    }
}

impl <T: MixedNum + MixedNumSigned + MixedSqrt + MixedOps + MixedAbs + MixedPowi + MixedAtan> ToPolar<T> for Cartesian<T>
{
    /// Complex<T> to Polar<T>.
    #[inline(always)]
    fn to_polar( &self ) -> Polar<T>
    {
        return ops::to_polar(*self);
    }
}

impl <T: MixedNum + MixedZero> MixedZero for Cartesian<T>
{
    /// Return the zero value of type Self.
    #[inline(always)]
    fn mixed_zero() -> Self {
        return Self{re:T::mixed_zero(), im:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedZero + MixedOne> MixedOne for Cartesian<T>
{
    /// Return the zero value of type Self.
    #[inline(always)]
    fn mixed_one() -> Self {
        return Self{re:T::mixed_one(), im:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedZero> MixedComplexConversion<T>  for Cartesian<T>
{
    /// Type cast from real number T to Complex<T>.
    fn mixed_to_complex( number:T ) -> Self {
        return Self{re:number, im:T::mixed_zero()};
    }
}

impl <T: MixedNum + MixedNumSigned + MixedSqrt + MixedPowi + MixedOps> Mag<T> for Cartesian<T>
{
    /// Magnitude of the complex number.
    #[inline(always)]
    fn mag( &self ) -> T
    {
        return (self.re.mixed_powi(2)+self.im.mixed_powi(2)).mixed_sqrt();
    }
    /// Magnitude of the complex number.
    #[inline(always)]
    fn abs( &self ) -> T
    {
        return self.mag();
    }
}

impl <T: MixedNum + MixedNumSigned + MixedSqrt + MixedPowi + MixedOps + MixedZero> MixedAbs for Cartesian<T>
{
    fn mixed_abs( &self ) -> Self
    {
        return Self::new(self.mag(), T::mixed_zero());
    }
}

impl <T: MixedNum + MixedNumSigned + MixedAtan> Arg<T> for Cartesian<T>
{
    /// Argument of the complex number.
    #[inline(always)]
    fn arg( &self ) -> T
    {
        return self.re.mixed_atan2(self.im);
    }

    /// Angle of the complex number.
    #[inline(always)]
    fn ang( &self ) -> T
    {
        return self.arg();
    }
}

impl <T: MixedNum + MixedNumSigned + MixedOps> core::ops::Mul<Cartesian<T>> for Cartesian<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        return ops::mul_cartesian(self, rhs);
    }
}

macro_rules! impl_core_ops_cartesian_for_cartesian{
    ( $T:ty ) => {
        impl <T: MixedNum + MixedNumSigned + MixedOps> core::ops::Mul<$T> for Cartesian<T> {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: $T) -> Cartesian<T> {
                return ops::mul_cartesian(self, *rhs);
            }
        }
    }
}

impl_core_ops_cartesian_for_cartesian!(&Cartesian<T>);
impl_core_ops_cartesian_for_cartesian!(&mut Cartesian<T>);



impl <T1: MixedNum + MixedOps, T2: MixedNum + MixedOps + MixedNumConversion<T1>> core::ops::Mul<T1> for Cartesian<T2> {
    type Output = Self;
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(1f32,2f32);
    /// 
    /// c_num = c_num*2f64;
    /// assert_eq!{ c_num.to_string(), "2+4i" };
    /// ``` 
    #[inline]
    fn mul(self, rhs: T1) -> Self {
        return Cartesian::new(self.re*T2::mixed_from_num(rhs), self.im*T2::mixed_from_num(rhs));
    }
}

macro_rules! impl_core_ops_polar_for_cartesian{
    ( $T:ty ) => {
        impl <T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedWrapPhase + MixedOps> core::ops::Mul<$T> for Cartesian<T> {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: $T) -> Self {
                let rhs_cartesian = rhs.to_cartesian();
                return ops::mul_cartesian(self, rhs_cartesian);
            }
        }
        
        impl <T: MixedNum + MixedNumSigned + MixedOps + MixedTrigonometry + MixedWrapPhase> core::ops::MulAssign<$T> for Cartesian<T> {
            #[inline]
            fn mul_assign(&mut self, rhs: $T) {
                let temp = *self* rhs.to_cartesian();
                self.re = temp.re;
                self.im = temp.im;
            }
        }
    }
}

impl_core_ops_polar_for_cartesian!(Polar<T>);
impl_core_ops_polar_for_cartesian!(&Polar<T>);
impl_core_ops_polar_for_cartesian!(&mut Polar<T>);


impl <T1: MixedNum + MixedOps + MixedZero, T2: MixedNum + MixedOps + MixedNumConversion<T1>> core::ops::Add<T1> for Cartesian<T2> {
    type Output = Self;
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(1f32,2f32);
    /// 
    /// c_num = c_num+2f64;
    /// assert_eq!{ c_num.to_string(), "3+2i" };
    /// ```
    #[inline]
    fn add(self, rhs: T1) -> Self {
        return Cartesian::new(self.re+T2::mixed_from_num(rhs), self.im);
    }
}

impl <T1: MixedNum + MixedOps + MixedZero, T2: MixedOps + MixedNumConversion<T1>> core::ops::AddAssign<T1> for Cartesian<T2> {
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(1f32,2f32);
    /// 
    /// c_num += 2f64;
    /// assert_eq!{ c_num.to_string(), "3+2i" };
    /// ```
    #[inline]
    fn add_assign(&mut self, rhs: T1) {
        self.re =  self.re+T2::mixed_from_num(rhs);
    }
}

macro_rules! impl_core_ops_add_sub_for_cartesian{
    ( $T:ty ) => {
        impl <T: MixedNum + MixedNumSigned + MixedOps + MixedZero> core::ops::Add<$T> for Cartesian<T> {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $T) -> Self {
                return Cartesian::new(self.re+rhs.re, self.im+rhs.im);
            }
        }

        impl <T: MixedNum + MixedNumSigned + MixedOps + MixedZero> core::ops::AddAssign<$T> for Cartesian<T> {
            #[inline]
            fn add_assign(&mut self, rhs: $T) {
                self.re =  self.re+rhs.re;
                self.im =  self.re+rhs.im;
            }
        }
        
        impl <T: MixedNum + MixedNumSigned + MixedOps + MixedZero> core::ops::Sub<$T> for Cartesian<T> {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: $T) -> Self {
                return Cartesian::new(self.re-rhs.re, self.im-rhs.im);
            }
        }

        impl <T: MixedNum + MixedNumSigned + MixedOps + MixedZero> core::ops::SubAssign<$T> for Cartesian<T> {
            #[inline]
            fn sub_assign(&mut self, rhs: $T) {
                self.re =  self.re-rhs.re;
                self.im =  self.re-rhs.im;
            }
        }
    }
}

impl_core_ops_add_sub_for_cartesian!(Cartesian<T>);
impl_core_ops_add_sub_for_cartesian!(&Cartesian<T>);
impl_core_ops_add_sub_for_cartesian!(&mut Cartesian<T>);

impl_core_ops_add_sub_for_cartesian!(num::Complex<T>);
impl_core_ops_add_sub_for_cartesian!(&num::Complex<T>);
impl_core_ops_add_sub_for_cartesian!(&mut num::Complex<T>);


impl <T1: MixedNum + MixedOps, T2: MixedOps + MixedNumConversion<T1>> core::ops::Sub<T1> for Cartesian<T2> {
    type Output = Self;
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(1f32,2f32);
    /// 
    /// c_num = c_num-2f64;
    /// assert_eq!{ c_num.to_string(), "-1+2i" };
    /// ```
    #[inline]
    fn sub(self, rhs: T1) -> Self {
        return Cartesian::new(self.re-T2::mixed_from_num(rhs), self.im);
    }
}

impl <T1: MixedNum + MixedOps, T2: MixedOps + MixedNumConversion<T1>> core::ops::SubAssign<T1> for Cartesian<T2> {
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(1f32,2f32);
    /// 
    /// c_num = c_num-2f64;
    /// assert_eq!{ c_num.to_string(), "-1+2i" };
    /// ```
    #[inline]
    fn sub_assign(&mut self, rhs: T1) {
        self.re =  self.re-T2::mixed_from_num(rhs);
    }
}

macro_rules! impl_core_ops_div_cartesian_for_cartesian{
    ( $T:ty ) => {
        impl <T: MixedNum + MixedNumSigned + MixedOps + MixedPowi> core::ops::Div<$T> for Cartesian<T> {
            type Output = Self;
            fn div(self, rhs: $T) -> Self {
                //  ((a,bi))/((c,di))=((ac+bd)/(c^2+d^2),(bc-ad)/(c^2+d^2) i)
        
                let a = self.re;
                let b = self.im;
                let c = rhs.re;
                let d = rhs.re;
        
                return Cartesian::new((a*c+b*d)/(c.mixed_powi(2)+d.mixed_powi(2)), (b*c-a*d)/(c.mixed_powi(2)+d.mixed_powi(2)));
            }
        }

        impl <T: MixedNum + MixedNumSigned + MixedOps + MixedZero + MixedPowi> core::ops::DivAssign<$T> for Cartesian<T> {
            fn div_assign(&mut self, rhs: $T) {
                //  ((a,bi))/((c,di))=((ac+bd)/(c^2+d^2),(bc-ad)/(c^2+d^2) i)
        
                let a = self.re;
                let b = self.im;
                let c = rhs.re;
                let d = rhs.re;
        
                self.re = (a*c+b*d)/(c.mixed_powi(2)+d.mixed_powi(2));
                self.im = (b*c-a*d)/(c.mixed_powi(2)+d.mixed_powi(2));
            }
        }
    }
}

impl_core_ops_div_cartesian_for_cartesian!(Cartesian<T>);
impl_core_ops_div_cartesian_for_cartesian!(&Cartesian<T>);
impl_core_ops_div_cartesian_for_cartesian!(&mut Cartesian<T>);

impl_core_ops_div_cartesian_for_cartesian!(num::Complex<T>);
impl_core_ops_div_cartesian_for_cartesian!(&num::Complex<T>);
impl_core_ops_div_cartesian_for_cartesian!(&mut num::Complex<T>);


impl <T1: MixedNumSigned + MixedOps + MixedZero, T2: MixedReal + MixedOps + MixedNumConversion<T1>> core::ops::Div<T1> for Cartesian<T2> {
    type Output = Self;
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(-2f32,4f32);
    /// 
    /// c_num = c_num/2f64;
    /// assert_eq!{ c_num.to_string(), "-1+2i" };
    /// ```
    #[inline]
    fn div(self, rhs: T1) -> Self {
        if rhs == T1::mixed_zero() {
            return Cartesian::new(T2::mixed_max_value(), T2::mixed_max_value());
        }
        return Cartesian::new(self.re/T2::mixed_from_num(rhs), self.im/T2::mixed_from_num(rhs));
    }
}

impl <T1: MixedNumSigned + MixedOps + MixedZero, T2: MixedReal + MixedOps + MixedNumConversion<T1>> core::ops::DivAssign<T1> for Cartesian<T2> {
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(-2f32,4f32);
    /// 
    /// c_num /= 2f64;
    /// assert_eq!{ c_num.to_string(), "-1+2i" };
    /// ```
    #[inline]
    fn div_assign(&mut self, rhs: T1) {
        if rhs == T1::mixed_zero() {
            self.re =T2::mixed_max_value();
            self.im =T2::mixed_max_value();
        }
        self.re = self.re/T2::mixed_from_num(rhs);
        self.im = self.im/T2::mixed_from_num(rhs);
    }
}

impl <T: MixedComplex + NewFromCartesian<T2>, T2: MixedNum + MixedNumSigned> Conj<T> for Cartesian<T2>
{
    /// Complex Conjugate of T.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(-2f32,-4f32);
    /// 
    /// c_num = c_num.conj();
    /// assert_eq!{ c_num.to_string(), "-2+4i" };
    /// ```
    fn conj( &self ) -> T {
        return T::new_from_cartesian(self.re, -self.im);
    }
}