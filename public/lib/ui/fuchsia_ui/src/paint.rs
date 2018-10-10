// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::color::Color;
use crate::geometry::{Coord, Rect, Size};

pub struct PaintCtx {}

impl PaintCtx {
    pub fn fill_rect(&mut self, r: &Rect, color: &Color) {}

    pub fn size(&self) -> Size {
        Size::new(200.0, 200.0)
    }

    pub fn clear(&mut self) {}

    pub fn present(&mut self) {}
}
