// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2023 lacklustr@protonmail.com https://github.com/eadf

// This file is part of vector-traits.

#[cfg(test)]
mod tests;

#[cfg(feature = "approx")]
use crate::SimpleApprox;
use crate::{GenericScalar, GenericVector2, GenericVector3, HasXY, HasXYZ};
pub use ::cgmath::{InnerSpace, MetricSpace, Vector2, Vector3};
use approx::{abs_diff_eq, ulps_eq};
use num_traits::Zero;

macro_rules! impl_cgmath_vector2 {
    ($vec2_type:ty, $vec3_type:ty) => {
        impl HasXY for $vec2_type {
            type Scalar = <$vec2_type as cgmath::VectorSpace>::Scalar;
            #[inline(always)]
            fn new_2d(x: Self::Scalar, y: Self::Scalar) -> Self {
                <$vec2_type>::new(x, y)
            }
            #[inline(always)]
            fn x(self) -> Self::Scalar {
                self.x
            }
            #[inline(always)]
            fn x_mut(&mut self) -> &mut Self::Scalar {
                &mut self.x
            }
            #[inline(always)]
            fn y(self) -> Self::Scalar {
                self.y
            }
            #[inline(always)]
            fn y_mut(&mut self) -> &mut Self::Scalar {
                &mut self.y
            }
        }

        #[cfg(feature = "approx")]
        impl SimpleApprox for $vec2_type {
            type AScalar = <$vec2_type as cgmath::VectorSpace>::Scalar;
            #[inline(always)]
            fn is_ulps_eq(self, other: Self) -> bool {
                ulps_eq!(self.x, other.x) && ulps_eq!(self.y, other.y)
            }
            #[inline(always)]
            fn is_abs_diff_eq(self, other: Self) -> bool {
                abs_diff_eq!(self.x, other.x) && abs_diff_eq!(self.y, other.y)
            }
        }

        impl GenericVector2 for $vec2_type {
            type Vector3 = $vec3_type;

            #[inline(always)]
            fn to_3d(self, z: Self::Scalar) -> Self::Vector3 {
                <$vec3_type>::new(self.x, self.y, z)
            }
            #[inline(always)]
            fn magnitude(self) -> Self::Scalar {
                InnerSpace::magnitude(self)
            }
            #[inline(always)]
            fn magnitude_sq(self) -> Self::Scalar {
                InnerSpace::magnitude2(self)
            }
            #[inline(always)]
            fn dot(self, rhs: Self) -> Self::Scalar {
                <$vec2_type as cgmath::InnerSpace>::dot(self, rhs)
            }
            #[inline(always)]
            fn normalize(self) -> Self {
                <$vec2_type as cgmath::InnerSpace>::normalize(self)
            }
            #[inline(always)]
            fn perp_dot(self, other: Self) -> Self::Scalar {
                self.x * other.y - self.y * other.x
            }
            #[inline(always)]
            fn safe_normalize(self) -> Option<Self> {
                let l = InnerSpace::magnitude(self);
                if l.is_zero() {
                    None
                } else {
                    Some(self / l)
                }
            }
            #[inline(always)]
            fn distance(self, rhs: Self) -> Self::Scalar {
                <$vec2_type as cgmath::MetricSpace>::distance(self, rhs)
            }
            #[inline(always)]
            fn distance_sq(self, rhs: Self) -> Self::Scalar {
                <$vec2_type>::distance2(self, rhs)
            }
        }
    };
}

impl_cgmath_vector2!(Vector2<f32>, Vector3<f32>);
impl_cgmath_vector2!(Vector2<f64>, Vector3<f64>);

macro_rules! impl_cgmath_vector3 {
    ($vec3_type:ty, $vec2_type:ty) => {
        impl HasXY for $vec3_type {
            type Scalar = <$vec3_type as cgmath::VectorSpace>::Scalar;
            #[inline(always)]
            fn new_2d(x: Self::Scalar, y: Self::Scalar) -> Self {
                <$vec3_type>::new(x, y, Self::Scalar::ZERO)
            }
            #[inline(always)]
            fn x(self) -> Self::Scalar {
                self.x
            }
            #[inline(always)]
            fn x_mut(&mut self) -> &mut Self::Scalar {
                &mut self.x
            }
            #[inline(always)]
            fn y(self) -> Self::Scalar {
                self.y
            }
            #[inline(always)]
            fn y_mut(&mut self) -> &mut Self::Scalar {
                &mut self.y
            }
        }

        impl HasXYZ for $vec3_type {
            #[inline(always)]
            fn new_3d(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self {
                <$vec3_type>::new(x, y, z)
            }
            #[inline(always)]
            fn z(self) -> Self::Scalar {
                self.z
            }
            #[inline(always)]
            fn z_mut(&mut self) -> &mut Self::Scalar {
                &mut self.z
            }
        }
        #[cfg(feature = "approx")]
        impl SimpleApprox for $vec3_type {
            type AScalar = <$vec3_type as cgmath::VectorSpace>::Scalar;
            #[inline(always)]
            fn is_ulps_eq(self, other: Self) -> bool {
                ulps_eq!(self.x, other.x) && ulps_eq!(self.y, other.y) && ulps_eq!(self.z, other.z)
            }
            #[inline(always)]
            fn is_abs_diff_eq(self, other: Self) -> bool {
                abs_diff_eq!(self.x, other.x)
                    && abs_diff_eq!(self.y, other.y)
                    && abs_diff_eq!(self.z, other.z)
            }
        }
        impl GenericVector3 for $vec3_type {
            type Vector2 = $vec2_type;
            #[inline(always)]
            fn to_2d(&self) -> Self::Vector2 {
                Self::Vector2::new(self.x, self.y)
            }
            #[inline(always)]
            fn magnitude(self) -> Self::Scalar {
                InnerSpace::magnitude(self)
            }
            #[inline(always)]
            fn magnitude_sq(self) -> Self::Scalar {
                InnerSpace::magnitude2(self)
            }
            #[inline(always)]
            fn normalize(self) -> Self {
                InnerSpace::normalize(self)
            }
            #[inline(always)]
            fn safe_normalize(self) -> Option<Self> {
                let l = InnerSpace::magnitude(self);
                if l.is_zero() {
                    None
                } else {
                    Some(self / l)
                }
            }
            #[inline(always)]
            fn dot(self, rhs: Self) -> Self::Scalar {
                <$vec3_type as cgmath::InnerSpace>::dot(self, rhs)
            }
            #[inline(always)]
            fn cross(self, rhs: Self) -> Self {
                <$vec3_type>::cross(self, rhs)
            }
            #[inline(always)]
            fn distance(self, rhs: Self) -> Self::Scalar {
                <$vec3_type as cgmath::MetricSpace>::distance(self, rhs)
            }
            #[inline(always)]
            fn distance_sq(self, rhs: Self) -> Self::Scalar {
                <$vec3_type>::distance2(self, rhs)
            }
        }
    };
}

impl_cgmath_vector3!(Vector3<f32>, Vector2<f32>);
impl_cgmath_vector3!(Vector3<f64>, Vector2<f64>);
