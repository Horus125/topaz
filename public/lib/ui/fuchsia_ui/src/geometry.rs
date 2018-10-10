// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate euclid;

pub type Coord = f32;
pub type Bounds = euclid::Rect<Coord>;
pub type Rect = euclid::Rect<Coord>;
pub type Point = euclid::Point2D<Coord>;
pub type Size = euclid::Size2D<Coord>;

pub fn new_empty_bounds() -> Bounds {
    Bounds::new(Point::new(0.0, 0.0), Size::new(0.0, 0.0))
}

pub fn size_to_point(size: &Size) -> Point {
    Point::new(size.width, size.height)
}
