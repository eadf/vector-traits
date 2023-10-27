// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2023 lacklustr@protonmail.com https://github.com/eadf

// This file is part of vector-traits.

#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    nonstandard_style,
    unused,
    future_incompatible,
    non_camel_case_types,
    unused_parens,
    non_upper_case_globals,
    unused_qualifications,
    unused_results,
    unused_imports,
    unused_variables,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    elided_lifetimes_in_paths
)]
#![warn(clippy::explicit_into_iter_loop)]

use num_traits::{float::FloatCore, real::Real, AsPrimitive, FromPrimitive, Signed, ToPrimitive};
use std::{
    fmt::{Debug, Display, LowerExp},
    ops::{Add, AddAssign, DivAssign, Index, MulAssign, Neg, Sub, SubAssign},
};

#[cfg(feature = "cgmath")]
pub mod cgmath_impl;
#[cfg(feature = "glam")]
pub mod glam_impl;
#[cfg(test)]
mod tests;

/// A trait meant to to represent f32 or f64
pub trait GenericScalar
where
    Self: Display
        + Debug
        + Real
        + FloatCore
        + FromPrimitive
        + ToPrimitive
        + MulAssign
        + DivAssign
        + AddAssign
        + SubAssign
        + Default
        + std::str::FromStr
        + Sync
        + Send
        + Into<f64>
        + From<f32>
        + From<u16>
        + From<i16>
        + From<i8>
        + From<u8>
        + Neg<Output = Self>
        + Signed
        + LowerExp
        + AsPrimitive<f64>
        + AsPrimitive<f32>
        + AsPrimitive<usize>
        + AsPrimitive<isize>
        + AsPrimitive<u64>
        + AsPrimitive<i64>
        + AsPrimitive<u32>
        + AsPrimitive<i32>
        + AsPrimitive<u16>
        + AsPrimitive<i16>
        + AsPrimitive<u8>
        + AsPrimitive<i8>
        + approx::UlpsEq<Epsilon = Self>,
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const EPSILON: Self;
    fn is_normal(self) -> bool;
    fn is_finite(self) -> bool;
    fn clamp(self, min: Self, max: Self) -> Self;
}

/// A workaround for Rust's limitations where external traits cannot be implemented for external types.
///
/// The `Approx` trait provides methods for performing approximate equality comparisons on types.
/// It serves as a workaround for Rust's limitations, allowing you to implement approximate
/// equality checks for types not originally designed with this capability.
///
/// This trait leverages the `approx` crate and its traits to perform approximate equality
/// comparisons. The methods in this trait are wrappers around the corresponding methods provided
/// by the `approx` crate.
///
pub trait Approx: HasXY {
    /// Checks if two instances are nearly equal within a specified tolerance in ULPs (Units in the Last Place).
    ///
    /// This method delegates to the `approx::UlpsEq::ulps_eq` method, performing approximate equality checks
    /// one time per coordinate axis.
    fn is_ulps_eq(
        self,
        other: Self,
        epsilon: <Self::Scalar as approx::AbsDiffEq>::Epsilon,
        max_ulps: u32,
    ) -> bool;
    /// Checks if two instances are nearly equal within a specified absolute difference tolerance.
    ///
    /// This method delegates to the `approx::AbsDiffEq::abs_diff_eq` method, performing approximate equality checks
    /// one time per coordinate axis.
    fn is_abs_diff_eq(
        self,
        other: Self,
        epsilon: <Self::Scalar as approx::AbsDiffEq>::Epsilon,
    ) -> bool;
}

/// A generic two-dimensional vector trait, designed for flexibility in precision.
///
/// The `GenericVector2` trait abstracts over two-dimensional vectors, allowing for easy
/// transition between different precisions (e.g., `f32` and `f64`) without necessitating
/// significant codebase modifications. It provides the common operations one would expect
/// for 2D vectors, such as dot products, cross products, and normalization.
///
/// Implementors of this trait can benefit from the ability to switch between different
/// precision representations seamlessly, making it ideal for applications where varying
/// precision levels might be desirable at different stages or configurations.
///
/// The associated `Scalar` type represents the scalar type (e.g., `f32` or `f64`) used
/// by the vector, and `Vector3` is the corresponding three-dimensional vector type.
///
/// Note: The actual trait functionality might vary based on the concrete implementations.
pub trait GenericVector2:
    HasXY
    + Approx
    + PartialEq
    + AddAssign
    + Neg<Output = Self>
    + Sub<Self, Output = Self>
    + std::ops::Mul<Self::Scalar, Output = Self>
    + std::ops::Div<Self::Scalar, Output = Self>
    + Add<Self, Output = Self>
    + Index<usize, Output = Self::Scalar>
{
    type Vector3: GenericVector3<Scalar = Self::Scalar, Vector2 = Self>;
    fn to_3d(self, z: Self::Scalar) -> Self::Vector3;
    fn magnitude(self) -> Self::Scalar;
    fn magnitude_sq(self) -> Self::Scalar;
    fn dot(self, other: Self) -> Self::Scalar;
    fn perp_dot(self, rhs: Self) -> Self::Scalar;
    fn distance(self, rhs: Self) -> Self::Scalar;
    fn distance_sq(self, rhs: Self) -> Self::Scalar;
    fn normalize(self) -> Self;
    fn safe_normalize(self) -> Option<Self>;
}

impl GenericScalar for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const THREE: Self = 3.0;
    const INFINITY: Self = <f32>::INFINITY;
    const NEG_INFINITY: Self = <f32>::NEG_INFINITY;
    const EPSILON: Self = <f32>::EPSILON;
    #[inline(always)]
    fn is_normal(self) -> bool {
        f32::is_normal(self)
    }
    #[inline(always)]
    fn is_finite(self) -> bool {
        f32::is_finite(self)
    }
    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self {
        f32::clamp(self, min, max)
    }
}

impl GenericScalar for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const THREE: Self = 3.0;
    const INFINITY: Self = <f64>::INFINITY;
    const NEG_INFINITY: Self = <f64>::NEG_INFINITY;
    const EPSILON: Self = <f64>::EPSILON;

    #[inline(always)]
    fn is_normal(self) -> bool {
        f64::is_normal(self)
    }
    #[inline(always)]
    fn is_finite(self) -> bool {
        f64::is_finite(self)
    }
    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self {
        f64::clamp(self, min, max)
    }
}

/// A basic two-dimensional vector trait, designed for flexibility in precision.
///
/// The `HasXY` trait abstracts over two-dimensional vectors, allowing for easy
/// transition between different precisions (e.g., `f32` and `f64`) without necessitating
/// significant codebase modifications. It only provides the most basic vector interface.
/// It is intended to be used in situations where you need a custom storage type of vectors.
/// For example a FFI type.
///
/// Implementors of this trait can benefit from the ability to switch between different
/// precision representations seamlessly, making it ideal for applications where varying
/// precision levels might be desirable at different stages or configurations.
///
/// The associated `Scalar` type represents the scalar type (e.g., `f32` or `f64`) used
/// by the vector.
///
pub trait HasXY: Sync + Send + Copy + Debug + Sized {
    type Scalar: GenericScalar;
    /// create a new instance of Self, note that this
    /// creates a 3d vector if the instanced type is a 3d type
    fn new_2d(x: Self::Scalar, y: Self::Scalar) -> Self;
    fn x(self) -> Self::Scalar;
    fn x_mut(&mut self) -> &mut Self::Scalar;
    fn set_x(&mut self, val: Self::Scalar);
    fn y(self) -> Self::Scalar;
    fn y_mut(&mut self) -> &mut Self::Scalar;
    fn set_y(&mut self, val: Self::Scalar);
}

/// A basic three-dimensional vector trait, designed for flexibility in precision.
///
/// The `HasXYZ` trait abstracts over three-dimensional vectors, allowing for easy
/// transition between different precisions (e.g., `f32` and `f64`) without necessitating
/// significant codebase modifications. It only provides the most basic vector interface.
/// It is intended to be used in situations where you need a custom storage type of vectors.
/// For example a FFI type.
///
/// Implementors of this trait can benefit from the ability to switch between different
/// precision representations seamlessly, making it ideal for applications where varying
/// precision levels might be desirable at different stages or configurations.
///
pub trait HasXYZ: HasXY {
    fn new_3d(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self;
    fn z(self) -> Self::Scalar;
    fn z_mut(&mut self) -> &mut Self::Scalar;
    fn set_z(&mut self, val: Self::Scalar);
}

/// A generic three-dimensional vector trait, designed for flexibility in precision.
///
/// The `GenericVector3` trait abstracts over three-dimensional vectors, allowing for easy
/// transition between different precisions (e.g., `f32` and `f64`) without necessitating
/// significant codebase modifications. It provides the common operations one would expect
/// for 3D vectors, such as dot products, cross products, and normalization.
///
/// Implementors of this trait can benefit from the ability to switch between different
/// precision representations seamlessly, making it ideal for applications where varying
/// precision levels might be desirable at different stages or configurations.
///
/// The associated `Scalar` type represents the scalar type (e.g., `f32` or `f64`) used
/// by the vector, and `Vector2` is the corresponding two-dimensional vector type.
///
/// Note: The actual trait functionality might vary based on the concrete implementations.
pub trait GenericVector3:
    HasXYZ
    + Approx
    + PartialEq
    + AddAssign
    + Neg<Output = Self>
    + Sub<Self, Output = Self>
    + std::ops::Mul<Self::Scalar, Output = Self>
    + std::ops::Div<Self::Scalar, Output = Self>
    + Add<Self, Output = Self>
    + Index<usize, Output = Self::Scalar>
{
    type Vector2: GenericVector2<Scalar = Self::Scalar, Vector3 = Self>;
    fn to_2d(&self) -> Self::Vector2;
    fn magnitude(self) -> Self::Scalar;
    fn magnitude_sq(self) -> Self::Scalar;
    fn dot(self, other: Self) -> Self::Scalar;
    fn cross(self, rhs: Self) -> Self;
    fn normalize(self) -> Self;
    fn safe_normalize(self) -> Option<Self>;
    fn distance(self, other: Self) -> Self::Scalar;
    fn distance_sq(self, rhs: Self) -> Self::Scalar;
}

pub use approx;
#[cfg(feature = "cgmath")]
pub use cgmath;
#[cfg(feature = "glam")]
pub use glam;
pub use num_traits;
