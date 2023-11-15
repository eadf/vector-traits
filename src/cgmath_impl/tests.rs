// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2023 lacklustr@protonmail.com https://github.com/eadf

// This file is part of vector-traits.

#[test]
fn test_xy() {
    crate::tests::tests::test_xy::<cgmath::Vector2<f32>>(1.0, 2.0);
    crate::tests::tests::test_xy::<cgmath::Vector2<f64>>(1.0, 2.0);
    crate::tests::tests::test_xy::<cgmath::Vector3<f32>>(1.0, 2.0);
    crate::tests::tests::test_xy::<cgmath::Vector3<f64>>(1.0, 2.0);
}

#[test]
fn test_xyz() {
    crate::tests::tests::test_xyz::<cgmath::Vector3<f32>>(1.0, 2.0, 3.0);
    crate::tests::tests::test_xyz::<cgmath::Vector3<f64>>(1.0, 2.0, 3.0);
}

#[test]
fn test_gxy() {
    crate::tests::tests::test_gxy::<cgmath::Vector2<f32>>(1.0, 2.0, 3.0);
    crate::tests::tests::test_gxy::<cgmath::Vector2<f64>>(1.0, 2.0, 3.0);
    crate::tests::tests::test_generic_xy::<cgmath::Vector2<f32>>(1.0, 2.0, 3.0, 0.00000001);
    crate::tests::tests::test_generic_xy::<cgmath::Vector2<f64>>(1.0, 2.0, 3.0, 0.0000000000001);
}

#[test]
fn test_gxyz() {
    crate::tests::tests::test_gxyz::<cgmath::Vector3<f32>>(1.0, 2.0, 3.0);
    crate::tests::tests::test_gxyz::<cgmath::Vector3<f64>>(1.0, 2.0, 3.0);
    crate::tests::tests::test_generic_xyz::<cgmath::Vector3<f32>>(1.0, 2.0, 3.0, 4.0, 0.0001);
    crate::tests::tests::test_generic_xyz::<cgmath::Vector3<f64>>(1.0, 2.0, 3.0, 4.0, 0.0000000000001);
}
