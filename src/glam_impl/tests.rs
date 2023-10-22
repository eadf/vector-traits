// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2023 lacklustr@protonmail.com https://github.com/eadf

// This file is part of vector-traits.

#[test]
fn test_xy() {
    crate::tests::test_xy::<glam::Vec2>(1.0, 2.0);
    crate::tests::test_xy::<glam::DVec2>(1.0, 2.0);
    crate::tests::test_xy::<glam::Vec3>(1.0, 2.0);
    crate::tests::test_xy::<glam::DVec3>(1.0, 2.0);
}

#[test]
fn test_xyz() {
    crate::tests::test_xyz::<glam::Vec3>(1.0, 2.0, 3.0);
    crate::tests::test_xyz::<glam::DVec3>(1.0, 2.0, 3.0);
}

#[test]
fn test_gxy() {
    crate::tests::test_gxy::<glam::Vec2>(1.0, 2.0, 3.0);
    crate::tests::test_gxy::<glam::DVec2>(1.0, 2.0, 3.0);
    crate::tests::test_generic_xy::<glam::Vec2>(1.0, 2.0, 3.0, 0.00000001);
    crate::tests::test_generic_xy::<glam::DVec2>(1.0, 2.0, 3.0, 0.0000000000001);
}

#[test]
fn test_gxyz() {
    crate::tests::test_gxyz::<glam::Vec3>(1.0, 2.0, 3.0);
    crate::tests::test_gxyz::<glam::DVec3>(1.0, 2.0, 3.0);
    crate::tests::test_generic_xyz::<glam::Vec3>(1.0, 2.0, 3.0, 4.0, 0.0001);
    crate::tests::test_generic_xyz::<glam::DVec3>(1.0, 2.0, 3.0, 4.0, 0.0000000000001);
}
