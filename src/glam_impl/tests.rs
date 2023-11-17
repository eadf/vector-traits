// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2023 lacklustr@protonmail.com https://github.com/eadf

// This file is part of vector-traits.

use crate::{HasXY, Vec2A};

#[test]
fn test_vec2a() {
    let a: Vec2A = (1.0, 2.0).into();
    let b: Vec2A = [1.0, 2.0].into();
    let c: Vec2A = glam::vec2(1.0, 2.0).into();
    let d: glam::Vec2 = c.into();

    assert_eq!(a.x(), b.x());
    assert_eq!(a.y(), b.y());
    assert_eq!(a.x(), c.x());
    assert_eq!(a.y(), c.y());
    assert_eq!(a.0, d);

    let a = Vec2A::new(1.0, 2.0);
    assert_eq!(a.x(), b.x());
    assert_eq!(a.y(), b.y());
}

#[test]
fn test_xy() {
    crate::tests::tests::test_xy::<glam::Vec2>(1.0, 2.0);
    crate::tests::tests::test_xy::<glam::DVec2>(1.0, 2.0);
    crate::tests::tests::test_xy::<Vec2A>(1.0, 2.0);
    crate::tests::tests::test_xy::<glam::Vec3A>(1.0, 2.0);
    crate::tests::tests::test_xy::<glam::Vec3>(1.0, 2.0);
    crate::tests::tests::test_xy::<glam::DVec3>(1.0, 2.0);
}

#[test]
fn test_xyz() {
    crate::tests::tests::test_xyz::<glam::Vec3>(1.0, 2.0, 3.0);
    crate::tests::tests::test_xyz::<glam::DVec3>(1.0, 2.0, 3.0);
    crate::tests::tests::test_xyz::<glam::Vec3A>(1.0, 2.0, 3.0);
}

#[test]
fn test_gxy() {
    crate::tests::tests::test_gxy::<glam::Vec2>(1.0, 2.0, 3.0);
    crate::tests::tests::test_gxy::<Vec2A>(1.0, 2.0, 3.0);
    crate::tests::tests::test_gxy::<glam::DVec2>(1.0, 2.0, 3.0);
    crate::tests::tests::test_generic_xy::<glam::Vec2>(1.0, 2.0, 3.0, 0.00000001);
    crate::tests::tests::test_generic_xy::<Vec2A>(1.0, 2.0, 3.0, 0.00000001);
    crate::tests::tests::test_generic_xy::<glam::DVec2>(1.0, 2.0, 3.0, 0.0000000000001);
}

#[test]
fn test_gxyz() {
    crate::tests::tests::test_gxyz::<glam::Vec3>(1.0, 2.0, 3.0);
    crate::tests::tests::test_gxyz::<glam::Vec3A>(1.0, 2.0, 3.0);
    crate::tests::tests::test_gxyz::<glam::DVec3>(1.0, 2.0, 3.0);
    crate::tests::tests::test_generic_xyz::<glam::Vec3>(1.0, 2.0, 3.0, 4.0, 0.0001);
    crate::tests::tests::test_generic_xyz::<glam::Vec3A>(1.0, 2.0, 3.0, 4.0, 0.0001);
    crate::tests::tests::test_generic_xyz::<glam::DVec3>(1.0, 2.0, 3.0, 4.0, 0.0000000000001);
}
