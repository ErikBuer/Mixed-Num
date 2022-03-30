// This file is an edited version of the lib.rs file of the num-complex crate.
// The lisence of this particular code section is included in the LISENCE file.

use crate::traits::*;
use crate::Cartesian;
use core::fmt;


macro_rules! write_complex {
    ($f:ident, $t:expr, $prefix:expr, $re:expr, $im:expr) => {{
        let abs_re = if $re < T::mixed_zero() {
            T::mixed_zero() - $re.clone()
        } else {
            $re.clone()
        };
        let abs_im = if $im < T::mixed_zero() {
            T::mixed_zero() - $im.clone()
        } else {
            $im.clone()
        };

        return if let Some(prec) = $f.precision() {
            fmt_re_im(
                $f,
                $re < T::mixed_zero(),
                $im < T::mixed_zero(),
                format_args!(concat!("{:.1$", $t, "}"), abs_re, prec),
                format_args!(concat!("{:.1$", $t, "}"), abs_im, prec),
            )
        } else {
            fmt_re_im(
                $f,
                $re < T::mixed_zero(),
                $im < T::mixed_zero(),
                format_args!(concat!("{:", $t, "}"), abs_re),
                format_args!(concat!("{:", $t, "}"), abs_im),
            )
        };

        fn fmt_re_im(
            f: &mut core::fmt::Formatter<'_>,
            re_neg: bool,
            im_neg: bool,
            real: core::fmt::Arguments<'_>,
            imag: core::fmt::Arguments<'_>,
        ) -> core::fmt::Result {
            let prefix = if f.alternate() { $prefix } else { "" };
            let sign = if re_neg {
                "-"
            } else if f.sign_plus() {
                "+"
            } else {
                ""
            };

            if im_neg {
                fmt_complex(
                    f,
                    format_args!(
                        "{}{pre}{re}-{pre}{im}i",
                        sign,
                        re = real,
                        im = imag,
                        pre = prefix
                    ),
                )
            } else {
                fmt_complex(
                    f,
                    format_args!(
                        "{}{pre}{re}+{pre}{im}i",
                        sign,
                        re = real,
                        im = imag,
                        pre = prefix
                    ),
                )
            }
        }

        #[cfg(feature = "std")]
        // Currently, we can only apply width using an intermediate `String` (and thus `std`)
        fn fmt_complex(f: &mut fmt::Formatter<'_>, complex: fmt::Arguments<'_>) -> fmt::Result {
            use std::string::ToString;
            if let Some(width) = f.width() {
                write!(f, "{0: >1$}", complex.to_string(), width)
            } else {
                write!(f, "{}", complex)
            }
        }

        #[cfg(not(feature = "std"))]
        fn fmt_complex(f: &mut core::fmt::Formatter<'_>, complex: core::fmt::Arguments<'_>) -> core::fmt::Result {
            write!(f, "{}", complex)
        }
    }};
}

impl<T> fmt::Display for Cartesian<T>
where
    T: fmt::Display + PartialOrd + Clone + MixedZero + MixedNum + core::ops::Sub<Output = T>,
{
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(-2f32,4f32);
    /// assert_eq!{ c_num.to_string(), "-2+4i" };
    /// let mut c_num = Cartesian::new(2f32,-4f32);
    /// assert_eq!{ c_num.to_string(), "2-4i" };
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_complex!(f, "", "", self.re, self.im)
    }
}

impl<T> fmt::LowerExp for Cartesian<T>
where
    T: fmt::LowerExp + PartialOrd + Clone + MixedZero + MixedNum + core::ops::Sub<Output = T>,
{
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(2e9f32,-4f32);
    /// assert_eq!{ format!("{:e}", c_num), "2e9-4e0i" };
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_complex!(f, "e", "", self.re, self.im)
    }
}

impl<T> fmt::UpperExp for Cartesian<T>
where
    T: fmt::UpperExp + PartialOrd + Clone + MixedZero + MixedNum + core::ops::Sub<Output = T>,
{
    /// ## Example
    /// 
    /// ```
    /// use mixed_num::*;
    /// use mixed_num::traits::*;
    /// 
    /// let mut c_num = Cartesian::new(2e9f32,-4f32);
    /// assert_eq!{ format!("{:E}", c_num), "2E9-4E0i" };
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_complex!(f, "E", "", self.re, self.im)
    }
}

impl<T> fmt::LowerHex for Cartesian<T>
where
    T: fmt::LowerHex + PartialOrd + Clone + MixedZero + MixedNum + core::ops::Sub<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_complex!(f, "x", "0x", self.re, self.im)
    }
}

impl<T> fmt::UpperHex for Cartesian<T>
where
    T: fmt::UpperHex + PartialOrd + Clone + MixedZero + MixedNum + core::ops::Sub<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_complex!(f, "X", "0x", self.re, self.im)
    }
}

impl<T> fmt::Octal for Cartesian<T>
where
    T: fmt::Octal + PartialOrd + Clone + MixedZero + MixedNum + core::ops::Sub<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_complex!(f, "o", "0o", self.re, self.im)
    }
}

impl<T> fmt::Binary for Cartesian<T>
where
    T: fmt::Binary + PartialOrd + Clone + MixedZero + MixedNum + core::ops::Sub<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_complex!(f, "b", "0b", self.re, self.im)
    }
}