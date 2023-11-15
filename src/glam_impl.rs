// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2023 lacklustr@protonmail.com https://github.com/eadf

// This file is part of vector-traits.

#[cfg(test)]
mod tests;

use crate::{Approx, GenericScalar, GenericVector2, GenericVector3, HasXY, HasXYZ};

use approx::{AbsDiffEq, UlpsEq};
use num_traits::Zero;
use std::ops::{Add, AddAssign, Div, Index, Mul, Neg, Sub};

use glam::{vec2, vec3a, DVec2, DVec3, Vec2, Vec3, Vec3A};
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
    };
}
macro_rules! impl_approx2 {
    ($vec_type:tt) => {
        impl Approx for $vec_type {
            #[inline(always)]
            fn is_ulps_eq(
                self,
                other: Self,
                epsilon: <Self::Scalar as AbsDiffEq>::Epsilon,
                max_ulps: u32,
            ) -> bool {
                self.x().ulps_eq(&other.x(), epsilon, max_ulps)
                    && self.y().ulps_eq(&other.y(), epsilon, max_ulps)
            }
            #[inline(always)]
            fn is_abs_diff_eq(
                self,
                other: Self,
                epsilon: <Self::Scalar as AbsDiffEq>::Epsilon,
            ) -> bool {
                self.x().abs_diff_eq(&other.x(), epsilon)
                    && self.y().abs_diff_eq(&other.y(), epsilon)
            }
        }
    };
}

impl_vector2!(Vec2, f32, Vec3);
impl_approx2!(Vec2);
impl_vector2!(DVec2, f64, DVec3);
impl_approx2!(DVec2);

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
    };
}
macro_rules! impl_approx3 {
    ($vec_type:ty) => {
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
impl_approx3!(Vec3);
impl_vector3!(DVec3, f64, DVec2);
impl_approx3!(DVec3);

/// A wrapper around `Vec2` with zero runtime cost. Created to facilitate the implementation of the trait
/// `GenericVector3` for `Vec3A`. While not an ideal solution, it is the most suitable one identified.
/// Note that this type is only as aligned as Vec2 is.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2A(pub Vec2);

impl Vec2A {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl From<Vec2> for Vec2A {
    fn from(v:Vec2) -> Self {
        Self(v)
    }
}

impl From<Vec2A> for Vec2 {
    fn from(v:Vec2A) -> Self {
        v.0
    }
}

// Implement From for (f32, f32)
impl From<(f32, f32)> for Vec2A {
    fn from(tuple: (f32, f32)) -> Self {
        Vec2A(Vec2::new(tuple.0, tuple.1))
    }
}

// Implement From for [f32; 2]
impl From<[f32; 2]> for Vec2A {
    fn from(array: [f32; 2]) -> Self {
        Vec2A(Vec2::new(array[0], array[1]))
    }
}

impl HasXY for Vec2A {
    type Scalar = f32;
    #[inline(always)]
    fn new_2d(x: Self::Scalar, y: Self::Scalar) -> Self {
        Self(Vec2::new(x, y))
    }

    #[inline(always)]
    fn x(self) -> Self::Scalar {
        self.0.x
    }

    #[inline(always)]
    fn x_mut(&mut self) -> &mut Self::Scalar {
        &mut self.0.x
    }

    #[inline(always)]
    fn set_x(&mut self, val: Self::Scalar) {
        self.0.x = val;
    }

    #[inline(always)]
    fn y(self) -> Self::Scalar {
        self.0.y
    }

    #[inline(always)]
    fn y_mut(&mut self) -> &mut Self::Scalar {
        &mut self.0.y
    }

    #[inline(always)]
    fn set_y(&mut self, val: Self::Scalar) {
        self.0.y = val
    }
}
impl_approx2!(Vec2A);

impl HasXY for Vec3A {
    type Scalar = f32;
    #[inline(always)]
    fn new_2d(x: Self::Scalar, y: Self::Scalar) -> Self {
        vec3a(x, y, Self::Scalar::ZERO)
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
    fn set_x(&mut self, val: Self::Scalar) {
        self.x = val;
    }

    #[inline(always)]
    fn y(self) -> Self::Scalar {
        self.y
    }

    #[inline(always)]
    fn y_mut(&mut self) -> &mut Self::Scalar {
        &mut self.y
    }

    #[inline(always)]
    fn set_y(&mut self, val: Self::Scalar) {
        self.y = val
    }
}

impl HasXYZ for Vec3A {
    #[inline(always)]
    fn new_3d(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self {
        vec3a(x, y, z)
    }

    #[inline(always)]
    fn z(self) -> Self::Scalar {
        self.z
    }

    #[inline(always)]
    fn z_mut(&mut self) -> &mut Self::Scalar {
        &mut self.z
    }

    #[inline(always)]
    fn set_z(&mut self, val: Self::Scalar) {
        self.z = val
    }
}

impl GenericVector2 for Vec2A {
    type Vector3 = Vec3A;

    #[inline(always)]
    fn to_3d(self, z: Self::Scalar) -> Self::Vector3 {
        vec3a(self.0.x, self.0.y, z)
    }

    #[inline(always)]
    fn magnitude(self) -> Self::Scalar {
        self.0.length()
    }

    #[inline(always)]
    fn magnitude_sq(self) -> Self::Scalar {
        self.0.length_squared()
    }

    #[inline(always)]
    fn dot(self, other: Self) -> Self::Scalar {
        self.0.dot(other.0)
    }

    #[inline(always)]
    fn perp_dot(self, rhs: Self) -> Self::Scalar {
        self.0.perp_dot(rhs.0)
    }

    #[inline(always)]
    fn distance(self, rhs: Self) -> Self::Scalar {
        self.0.distance(rhs.0)
    }

    #[inline(always)]
    fn distance_sq(self, rhs: Self) -> Self::Scalar {
        self.0.distance_squared(rhs.0)
    }

    #[inline(always)]
    fn normalize(self) -> Self {
        Vec2A(self.0.normalize())
    }

    #[inline(always)]
    fn safe_normalize(self) -> Option<Self> {
        let l = self.0.length();
        (!l.is_zero()).then(|| Vec2A(self.0 / l))
    }
}

impl GenericVector3 for Vec3A {
    type Vector2 = Vec2A;

    #[inline(always)]
    fn to_2d(&self) -> Self::Vector2 {
        Vec2A(vec2(self.x, self.y))
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
    fn dot(self, other: Self) -> Self::Scalar {
        self.dot(other)
    }

    #[inline(always)]
    fn cross(self, rhs: Self) -> Self {
        self.cross(rhs)
    }

    #[inline(always)]
    fn normalize(self) -> Self {
        self.normalize()
    }

    #[inline(always)]
    fn safe_normalize(self) -> Option<Self> {
        let l = self.length();
        (!l.is_zero()).then(|| self / l)
    }

    #[inline(always)]
    fn distance(self, other: Self) -> Self::Scalar {
        self.distance(other)
    }

    #[inline(always)]
    fn distance_sq(self, rhs: Self) -> Self::Scalar {
        self.distance_squared(rhs)
    }
}

impl_approx3!(Vec3A);

impl Add for Vec2A {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vec2A(self.0 + rhs.0)
    }
}

impl Sub for Vec2A {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2A(self.0 - rhs.0)
    }
}

impl Index<usize> for Vec2A {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl AddAssign for Vec2A {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul<f32> for Vec2A {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2A(self.0 * rhs)
    }
}

impl Div<f32> for Vec2A {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Vec2A(self.0 / rhs)
    }
}

impl Neg for Vec2A {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec2A(-self.0)
    }
}
