use super::*;

mod ops;
pub use ops::*;

mod cartesian_impl;
pub use cartesian_impl::*;

mod polar_impl;
pub use polar_impl::*;

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Cartesian<T> {
    /// Real part.
    pub re: T,
    /// Imaginary part.
    pub im: T,
}

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Default)]
#[repr(C)]
/// Polar complex nuber.
pub struct Polar<T> {
    // Magnitude
    pub mag: T,
    // Angle [rad]
    pub ang: T,
}