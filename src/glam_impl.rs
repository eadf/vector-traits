// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2023 lacklustr@protonmail.com https://github.com/eadf

// This file is part of vector-traits.

#[cfg(test)]
mod tests;
use crate::{Approx, GenericScalar, GenericVector2, GenericVector3, HasXY, HasXYZ};
#[cfg(feature = "approx")]
use approx::{abs_diff_eq, ulps_eq};
use approx::{AbsDiffEq, UlpsEq};
use num_traits::Zero;

#[cfg(feature = "approx")]
use crate::SimpleApprox;
use glam::{DVec2, DVec3, Vec2, Vec3};
macro_rules! impl_vector2 {
    ($vec_type:tt, $scalar_type:ty, $vec3_type:ty) => {
        impl HasXY for $vec_type {
            type Scalar = $scalar_type;
            #[inline(always)]
            fn new_2d(x: Self::Scalar, y: Self::Scalar) -> Self {
                <$vec_type>::new(x, y)
            }
            #[inline(always)]
            fn x(self) -> Self::Scalar {
                self.x
            }
            #[inline(always)]
            fn set_x(&mut self, val: Self::Scalar) {
                self.x = val
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
            fn set_y(&mut self, val: Self::Scalar) {
                self.y = val
            }
            #[inline(always)]
            fn y_mut(&mut self) -> &mut Self::Scalar {
                &mut self.y
            }
        }

        impl GenericVector2 for $vec_type {
            type Vector3 = $vec3_type;

            #[inline(always)]
            fn to_3d(self, z: Self::Scalar) -> Self::Vector3 {
                <$vec3_type>::new(self.x, self.y, z)
            }
            #[inline(always)]
            fn magnitude(self) -> Self::Scalar {
                self.length()
            }
            #[inline(always)]
            fn magnitude_sq(self) -> Self::Scalar {
                self.length_squared()
            }
            #[inline(always)]
            fn dot(self, rhs: Self) -> Self::Scalar {
                <$vec_type>::dot(self, rhs)
            }
            #[inline(always)]
            fn perp_dot(self, rhs: Self) -> Self::Scalar {
                self.perp_dot(rhs)
            }
            #[inline(always)]
            fn normalize(self) -> Self {
                <$vec_type>::normalize(self)
            }
            #[inline(always)]
            fn safe_normalize(self) -> Option<Self> {
                let l = self.length();
                if l.is_zero() {
                    None
                } else {
                    Some(self / l)
                }
            }
            #[inline(always)]
            fn distance(self, rhs: Self) -> Self::Scalar {
                <$vec_type>::distance(self, rhs)
            }
            #[inline(always)]
            fn distance_sq(self, rhs: Self) -> Self::Scalar {
                <$vec_type>::distance_squared(self, rhs)
            }
        }
        impl Approx for $vec_type {
            #[inline(always)]
            fn is_ulps_eq(
                self,
                other: Self,
                epsilon: <Self::Scalar as AbsDiffEq>::Epsilon,
                max_ulps: u32,
            ) -> bool {
                self.x.ulps_eq(&other.x, epsilon, max_ulps)
                    && self.y.ulps_eq(&other.y, epsilon, max_ulps)
            }
            #[inline(always)]
            fn is_abs_diff_eq(
                self,
                other: Self,
                epsilon: <Self::Scalar as AbsDiffEq>::Epsilon,
            ) -> bool {
                self.x.abs_diff_eq(&other.x, epsilon) && self.y.abs_diff_eq(&other.y, epsilon)
            }
        }
    };
}

impl_vector2!(Vec2, f32, Vec3);
impl_vector2!(DVec2, f64, DVec3);

macro_rules! impl_vector3 {
    ($vec_type:ty, $scalar_type:ty, $vec2_type:ty) => {
        impl HasXY for $vec_type {
            type Scalar = $scalar_type;
            fn new_2d(x: Self::Scalar, y: Self::Scalar) -> Self {
                <$vec_type>::new(x, y, Self::Scalar::ZERO)
            }
            #[inline(always)]
            fn x(self) -> Self::Scalar {
                self.x
            }
            #[inline(always)]
            fn set_x(&mut self, val: Self::Scalar) {
                self.x = val
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
            fn set_y(&mut self, val: Self::Scalar) {
                self.y = val
            }
            #[inline(always)]
            fn y_mut(&mut self) -> &mut Self::Scalar {
                &mut self.y
            }
        }

        impl HasXYZ for $vec_type {
            #[inline(always)]
            fn new_3d(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self {
                <$vec_type>::new(x, y, z)
            }
            #[inline(always)]
            fn z(self) -> Self::Scalar {
                self.z
            }
            #[inline(always)]
            fn set_z(&mut self, val: Self::Scalar) {
                self.z = val
            }
            #[inline(always)]
            fn z_mut(&mut self) -> &mut Self::Scalar {
                &mut self.z
            }
        }

        #[cfg(feature = "approx")]
        impl SimpleApprox for $vec_type {
            type AScalar = $scalar_type;
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

        impl GenericVector3 for $vec_type {
            type Vector2 = $vec2_type;
            #[inline(always)]
            fn to_2d(&self) -> Self::Vector2 {
                <$vec2_type>::new(self.x, self.y)
            }
            #[inline(always)]
            fn magnitude(self) -> Self::Scalar {
                <$vec_type>::length(self)
            }
            #[inline(always)]
            fn magnitude_sq(self) -> Self::Scalar {
                <$vec_type>::length_squared(self)
            }
            #[inline(always)]
            fn normalize(self) -> Self {
                <$vec_type>::normalize(self)
            }
            #[inline(always)]
            fn safe_normalize(self) -> Option<Self> {
                let l = self.length();
                if l.is_zero() {
                    None
                } else {
                    Some(self / l)
                }
            }
            #[inline(always)]
            fn dot(self, rhs: Self) -> Self::Scalar {
                <$vec_type>::dot(self, rhs)
            }
            #[inline(always)]
            fn cross(self, rhs: Self) -> Self {
                <$vec_type>::cross(self, rhs)
            }
            #[inline(always)]
            fn distance(self, rhs: Self) -> Self::Scalar {
                <$vec_type>::distance(self, rhs)
            }
            #[inline(always)]
            fn distance_sq(self, rhs: Self) -> Self::Scalar {
                <$vec_type>::distance_squared(self, rhs)
            }
        }

        impl Approx for $vec_type {
            #[inline(always)]
            fn is_ulps_eq(
                self,
                other: Self,
                epsilon: <Self::Scalar as AbsDiffEq>::Epsilon,
                max_ulps: u32,
            ) -> bool {
                self.x.ulps_eq(&other.x, epsilon, max_ulps)
                    && self.y.ulps_eq(&other.y, epsilon, max_ulps)
                    && self.z.ulps_eq(&other.z, epsilon, max_ulps)
            }
            #[inline(always)]
            fn is_abs_diff_eq(
                self,
                other: Self,
                epsilon: <Self::Scalar as AbsDiffEq>::Epsilon,
            ) -> bool {
                self.x.abs_diff_eq(&other.x, epsilon)
                    && self.y.abs_diff_eq(&other.y, epsilon)
                    && self.z.abs_diff_eq(&other.z, epsilon)
            }
        }
    };
}

impl_vector3!(Vec3, f32, Vec2);
impl_vector3!(DVec3, f64, DVec2);
