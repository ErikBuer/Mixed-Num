use num::complex::Complex;
use super::*;

mod ops;
pub use ops::*;

impl <T: MixedNum + MixedNumSigned> MixedComplex for Complex<T>
{
    
}