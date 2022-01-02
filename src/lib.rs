//! 
//! No-STD abstraction layer enabling numerical functions to be implemented once, and simultaneously support both fixed and floating point types.
//! 

#![crate_name = "mixed_num"]
#![no_std]

use num::traits::float::FloatCore;

struct Mixed<T>{
    number:T
}

impl<T> Mixed<T> {
    /// Create a new mixed number.
    #[allow(dead_code)]
    fn new( number: T ) -> Self
    {
        Mixed {
            number: number,
        }
    }
}

pub trait TypeConversion<T> {
    fn from_num( number:T ) -> Self;
    //fn to_num( &self )      -> T;
}

impl TypeConversion<T> for Mixed<f32>
{
    fn from_num( number:T ) -> Self {
        return Mixed::new( number );
    }

}