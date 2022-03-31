# Release Notes

## Release 0.4.5 (2022-03-31)

- Implements `MixedNumConversion<T>` for conversion to/from real types and `Cartesian<T>`.

## Release 0.4.4 (2022-03-31)

- Implements `MixedExp` for `Cartesian<T>`.

## Release 0.4.3 (2022-03-30)

- Correction of git link in the cargo.toml file.

## Release 0.4.2 (2022-03-30)

- Improves `core::fmt` for `Cartesian<T>` by using a modified version of the [`write_complex`](https://docs.rs/num-complex/latest/src/num_complex/lib.rs.html#1123-1211) macro from num_complex (link to the original macro, at the time of writing).

## Release 0.4.1 (2022-03-28)

- Implements trats for cross type operations on `Cartesian<T>`.

## Release 0.4 (2022-03-27)

- Splits `MixedNum` into itself and `MixedReal` to enable use of traits with complex numbers.

## Release 0.3.0 (2022-03-27)

- Implements complex structs `Cartesian<T>` and `Polar<T>`.
- Implements `MixedNum` traits on above structs.
- Implements function for division of cartesian complex numbers.

## Release 0.2.0 (2022-03-20)

- Larger changes of which traits groups.
- Corects powi funciton for n⁰=1.
- Implements mixed num traits for intiger types.

## Release 0.1.9 (2022-03-13)

- Bumps dependencies

## Release 0.1.8 (2022-02-06)

- Implemented math methods from libm and cordic:
- asin, acos, exp, sincos and atan are implemented for fixed and float, using the libm and cordic crate.
- Implemented exp10, exp2, floor, ceil, cbrt, tan, sinh, cosh and tanh for float types, using the libm crate.
- Implemented log, log10 and log2 for float types, using the libm crate.
- Implemented decibel conversion for floaring point types.

## Release 0.1.7 (2022-02-06)

- Implemented `mixed_tau` method for the `MixedPi` trait.
- Corrected error in `trigonometry::sine`. Added a plotter test for inspection.

## Release 0.1.6 (2022-01-30)

- Implemented trait `MixedZero` and `MixedOne` traits.
- Implement mixed_from_num for u32 and u64.
- Added optional STD feature. This enables STDs implementation of the trigonometric functions and `sqrt`.

## Release 0.1.5 (2022-01-29)

- Removed dependency fixed_trigonomtry.
- Implemented trait `MixedSqrt` with the NIIRF approximation.
- Implemented `MixedTrigonometry` trait for signed types.
- Implemented `powi` function to enable removing the fixed_trigonometry dep.

## Release 0.1.4 (2022-01-03)

- Implemented trait `MixedNumSigned` for siged types.

## Release 0.1.3 (2022-01-02)

- Added `mixed_is_positive` and `mixed_is_negative`.

## Release 0.1.2 (2022-01-02)

- Changed `pi` to `mixed_pi` to avoid name conflict.
- Changed `sign` to `mixed_sign` to avoid name conflict.

## Release 0.1.1 (2022-01-02)

- Added support for unsigned fixed point numbers.

## Release 0.1.0 (2022-01-02)

- First release.

**Contributors**: ErikBuer
