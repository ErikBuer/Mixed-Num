//! 
//! No-STD abstraction layer enabling numerical functions to be implemented once, and simultaneously support both fixed and floating point types.
//! 

#![crate_name = "mixed_num"]
#![no_std]

pub trait MixedNum {
    fn from_num<T>( number:T ) -> Self;
    //fn to_num<T>( &self )      -> T;
    //fn zero()-> Self;
    //fn one() -> Self;
    //fn max() -> Self;
    //fn min() -> Self;
}

impl MixedNum for f32 {
    fn from_num<T>( number:T ) -> Self {
        return T as ;
    }
}