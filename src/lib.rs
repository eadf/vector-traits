// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2023 lacklustr@protonmail.com https://github.com/eadf

// This file is part of vector-traits.

use num_traits::{real::Real, Float, FromPrimitive};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, DivAssign, Index, MulAssign, Neg, Sub, SubAssign},
};

#[cfg(feature = "cgmath")]
pub mod cgmath_impl;
#[cfg(feature = "cgmath")]
pub use cgmath;

#[cfg(feature = "glam")]
pub mod glam_impl;
#[cfg(feature = "glam")]
pub use glam;

#[cfg(test)]
mod tests;

pub trait GenericScalar
where
    Self: Display
        + Debug
        + Real
        + FromPrimitive
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
        + Neg<Output = Self>
        + num_traits::Signed,
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const EPSILON: Self;
    /// this is only intended for 'safe' use, it uses 'as'
    fn to_f32(v: Self) -> f32;
    /// this is only intended for 'safe' use, it uses 'as'
    fn to_f64(v: Self) -> f64;
    /// this is only intended for 'safe' use, it uses 'as'
    fn to_u32(v: Self) -> u32;
    /// this is only intended for 'safe' use, it uses 'as'
    fn to_usize(v: Self) -> usize;
    /// this is only intended for 'safe' use, it uses 'as'
    fn from_f32(v: f32) -> Self;
    /// this is only intended for 'safe' use, it uses 'as'
    fn from_f64(v: f64) -> Self;
    /// this is only intended for 'safe' use, it uses 'as'
    fn from_u32(v: u32) -> Self;
    /// this is only intended for 'safe' use, it uses 'as'
    fn from_usize(v: usize) -> Self;
    fn is_normal(self) -> bool;
    fn is_finite(self) -> bool;
    fn clamp(self, min: Self, max: Self) -> Self;
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
    fn to_f32(v: Self) -> f32 {
        v
    }
    #[inline(always)]
    fn to_f64(v: Self) -> f64 {
        v as f64
    }
    #[inline(always)]
    fn to_u32(v: Self) -> u32 {
        v as u32
    }
    #[inline(always)]
    fn to_usize(v: Self) -> usize {
        v as usize
    }
    #[inline(always)]
    fn from_f32(v: f32) -> Self {
        v
    }
    #[inline(always)]
    fn from_f64(v: f64) -> Self {
        v as f32
    }
    #[inline(always)]
    fn from_u32(v: u32) -> Self {
        v as f32
    }
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        v as f32
    }
    #[inline(always)]
    fn is_normal(self) -> bool {
        Float::is_normal(self)
    }
    #[inline(always)]
    fn is_finite(self) -> bool {
        Float::is_finite(self)
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
    fn to_f32(v: Self) -> f32 {
        v as f32
    }
    #[inline(always)]
    fn to_f64(v: Self) -> f64 {
        v
    }
    #[inline(always)]
    fn to_u32(v: Self) -> u32 {
        v as u32
    }
    #[inline(always)]
    fn to_usize(v: Self) -> usize {
        v as usize
    }
    #[inline(always)]
    fn from_f32(v: f32) -> Self {
        v as f64
    }
    #[inline(always)]
    fn from_f64(v: f64) -> Self {
        v
    }
    #[inline(always)]
    fn from_u32(v: u32) -> Self {
        v as f64
    }
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        v as f64
    }
    #[inline(always)]
    fn is_normal(self) -> bool {
        Float::is_normal(self)
    }
    #[inline(always)]
    fn is_finite(self) -> bool {
        Float::is_finite(self)
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
    fn y(self) -> Self::Scalar;
    fn y_mut(&mut self) -> &mut Self::Scalar;
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

#[cfg(feature = "approx")]
pub use approx;
#[cfg(feature = "approx")]
#[allow(clippy::wrong_self_convention)]
/// We are not allowed to implement approx::UlpsEq on external types, so this is a work-around
pub trait SimpleApprox {
    type AScalar: GenericScalar + approx::UlpsEq;
    fn is_ulps_eq(self, other: Self) -> bool;
    fn is_abs_diff_eq(self, other: Self) -> bool;
}

pub use num_traits;
