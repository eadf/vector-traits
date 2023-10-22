// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2023 lacklustr@protonmail.com https://github.com/eadf

// This file is part of vector-traits.

use crate::{GenericScalar, GenericVector2, GenericVector3, HasXY, HasXYZ, SimpleApprox}; // GenericScalar, GenericVector2, GenericVector3};
use num_traits::real::Real;

pub fn test_xy<T: HasXY + SimpleApprox>(x: T::Scalar, y: T::Scalar) {
    let v0 = T::new_2d(x, y);
    assert_eq!(v0.x(), x);
    assert_eq!(v0.y(), y);
    let mut v1 = v0;
    assert!(v0.is_ulps_eq(v1));
    assert!(v0.is_abs_diff_eq(v1));
    let mult = 6.0.into();
    *v1.x_mut() = x * mult;
    *v1.y_mut() = y * mult;
    assert_eq!(v1.x(), x * mult);
    assert_eq!(v1.y(), y * mult);

    let n = T::Scalar::INFINITY;
    assert!(!n.is_normal());
    assert!(!n.is_finite());
    let a: T::Scalar = 5.0.into();
    let b: T::Scalar = 6.0.into();
    let c: T::Scalar = 8.0.into();

    assert_eq!(b.clamp(a, c), b);
    assert_eq!(a.clamp(b, c), b);
}

pub fn test_xyz<T: HasXYZ + SimpleApprox>(x: T::Scalar, y: T::Scalar, z: T::Scalar) {
    let v0 = T::new_3d(x, y, z);
    assert_eq!(v0.x(), x);
    assert_eq!(v0.y(), y);
    assert_eq!(v0.z(), z);

    let mult = 6.0.into();
    let mut v1 = v0;
    assert!(v0.is_ulps_eq(v1));
    assert!(v0.is_abs_diff_eq(v1));
    *v1.x_mut() = x * mult;
    *v1.y_mut() = y * mult;
    *v1.z_mut() = z * mult;
    assert_eq!(v1.x(), x * mult);
    assert_eq!(v1.y(), y * mult);
    assert_eq!(v1.z(), z * mult);
}

pub fn test_gxy<T: GenericVector2>(x: T::Scalar, y: T::Scalar, z: T::Scalar) {
    let v0 = T::new_2d(x, y);
    assert_eq!(v0.x(), x);
    assert_eq!(v0.y(), y);
    let mut v1 = v0;
    let mult = 6.0.into();
    *v1.x_mut() = x * mult;
    *v1.y_mut() = y * mult;
    assert_eq!(v1.x(), x * mult);
    assert_eq!(v1.y(), y * mult);
    let v2 = v0.to_3d(z) * mult;
    assert_eq!(v2.x(), x * mult);
    assert_eq!(v2.y(), y * mult);
    assert_eq!(v2.z(), z * mult);
    let v2 = (v0 * mult).to_3d(z).to_2d() / mult;
    assert_eq!(v2.x(), x);
    assert_eq!(v2.y(), y);

    assert_eq!(T::Scalar::from_u32(4), T::Scalar::from_usize(4));
    assert_eq!(T::Scalar::from_f32(4.0), T::Scalar::from_f64(4.0));

    let a: T::Scalar = 4.0.into();
    assert_eq!(T::Scalar::to_u32(a), 4_u32);
    assert_eq!(T::Scalar::to_usize(a), 4_usize);
    assert_eq!(T::Scalar::to_f32(a), 4_f32);
    assert_eq!(T::Scalar::to_f64(a), 4_f64);
}

pub fn test_generic_xy<T: GenericVector2>(
    x: T::Scalar,
    y: T::Scalar,
    z: T::Scalar,
    epsilon: T::Scalar,
) {
    let v0 = T::new_2d(x, y);
    assert_eq!(v0.x(), x);
    assert_eq!(v0.y(), y);

    let mut v1 = v0;
    let mult = T::Scalar::from_f64(6.0);
    *v1.x_mut() *= mult;
    *v1.y_mut() *= mult;
    assert_eq!(v1.x(), x * mult);
    assert_eq!(v1.y(), y * mult);

    let v2 = v0.to_3d(z) * mult;
    assert_eq!(v2.x(), x * mult);
    assert_eq!(v2.y(), y * mult);
    assert_eq!(v2.z(), z * mult);

    let v3 = (v0 * mult).to_3d(z).to_2d() / mult;
    assert_eq!(v3.x(), x);
    assert_eq!(v3.y(), y);

    // Test magnitude and magnitude_sq
    let magnitude = v0.magnitude();
    let magnitude_sq = v0.magnitude_sq();
    assert!(
        (magnitude * magnitude - magnitude_sq).abs() < epsilon,
        "{} != {}",
        magnitude * magnitude,
        magnitude_sq
    );

    // Test dot product
    let dot = v0.dot(v1);
    assert_eq!(dot, (x * x * mult + y * y * mult));

    // Test perp_dot (the result will vary based on specific types and values)
    let _perp_dot = v0.perp_dot(v1);

    // Test distance and distance_sq
    let distance = v0.distance(v1);
    let distance_sq = v0.distance_sq(v1);
    assert!(
        (distance * distance - distance_sq).abs() < epsilon,
        "{} != {}",
        distance * distance,
        distance_sq
    );

    // Test normalize
    let normalized = v0.normalize();
    assert!(
        (normalized.magnitude() - T::Scalar::ONE) < epsilon,
        "{} != {}",
        normalized.magnitude(),
        T::Scalar::ONE
    );

    // Test safe_normalize
    let safe_normalized = v0.safe_normalize();
    match safe_normalized {
        Some(v) => assert!(
            (v.magnitude() - T::Scalar::ONE) < epsilon,
            "{} != {}",
            v.magnitude(),
            T::Scalar::from(1.0)
        ),
        None => {}
    };

    let v0 = T::new_2d(T::Scalar::ZERO, T::Scalar::ZERO);
    assert!(v0.safe_normalize().is_none());
}

pub fn test_gxyz<T: GenericVector3>(x: T::Scalar, y: T::Scalar, z: T::Scalar) {
    let v0 = T::new_2d(x, y);
    assert_eq!(v0.x(), x);
    assert_eq!(v0.y(), y);
    let v1 = v0.to_2d();
    assert_eq!(v1.x(), x);
    assert_eq!(v1.y(), y);

    let mult = 6.0.into();
    let mut v1 = v0;
    *v1.x_mut() = x * mult;
    *v1.y_mut() = y * mult;
    *v1.z_mut() = z * mult;
    assert_eq!(v1.x(), x * mult);
    assert_eq!(v1.y(), y * mult);
    assert_eq!(v1.z(), z * mult);
}

pub fn test_generic_xyz<T: GenericVector3>(
    x: T::Scalar,
    y: T::Scalar,
    z: T::Scalar,
    _w: T::Scalar, // an extra scalar value for testing
    epsilon: T::Scalar,
) {
    let v0 = T::new_3d(x, y, z);
    assert_eq!(v0.x(), x);
    assert_eq!(v0.y(), y);
    assert_eq!(v0.z(), z);

    let mut v1 = v0;
    let mult = T::Scalar::from_f64(6.0);
    *v1.x_mut() *= mult;
    *v1.y_mut() *= mult;
    *v1.z_mut() *= mult;
    assert_eq!(v1.x(), x * mult);
    assert_eq!(v1.y(), y * mult);
    assert_eq!(v1.z(), z * mult);

    let v2 = (v0 * mult).to_2d();
    assert_eq!(v2.x(), x * mult);
    assert_eq!(v2.y(), y * mult);

    // Test magnitude and magnitude_sq
    let magnitude = v0.magnitude();
    let magnitude_sq = v0.magnitude_sq();
    assert!(
        (magnitude * magnitude - magnitude_sq).abs() < epsilon,
        "{} != {}",
        magnitude * magnitude,
        magnitude_sq
    );

    // Test dot product
    let dot = v0.dot(v1);
    assert_eq!(dot, (x * x * mult + y * y * mult + z * z * mult));

    // Test cross product
    let cross_product = v0.cross(v1);
    // Depending on values, cross product may vary, but in this case v0 and v1 are collinear
    assert_eq!(
        cross_product,
        T::new_3d(T::Scalar::ZERO, T::Scalar::ZERO, T::Scalar::ZERO)
    );

    // Test distance and distance_sq
    let distance = v0.distance(v1);
    let distance_sq = v0.distance_sq(v1);
    assert!(
        (distance * distance - distance_sq).abs() < epsilon,
        "{} != {}",
        distance * distance,
        distance_sq
    );

    // Test normalize
    let normalized = v0.normalize();
    assert!(
        (normalized.magnitude() - T::Scalar::ONE) < epsilon,
        "{} != {}",
        normalized.magnitude(),
        T::Scalar::ONE
    );

    // Test safe_normalize
    let safe_normalized = v0.safe_normalize();
    match safe_normalized {
        Some(v) => assert!(
            (v.magnitude() - T::Scalar::ONE) < epsilon,
            "{} != {}",
            v.magnitude(),
            T::Scalar::from(1.0)
        ),
        None => {}
    };
    let v0 = T::new_3d(T::Scalar::ZERO, T::Scalar::ZERO, T::Scalar::ZERO);
    assert!(v0.safe_normalize().is_none());
}