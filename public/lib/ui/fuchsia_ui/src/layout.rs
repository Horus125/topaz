// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::geometry::{Bounds, Coord, Point, Size};
use crate::Id;
use std::any::Any;

#[derive(Clone, Copy, Debug)]
pub struct BoxConstraints {
    pub min_width: Coord,
    pub max_width: Coord,
    pub min_height: Coord,
    pub max_height: Coord,
}

fn clamp(val: Coord, min: Coord, max: Coord) -> Coord {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

impl BoxConstraints {
    pub fn tight(size: Size) -> BoxConstraints {
        BoxConstraints {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }

    pub fn constrain(&self, size: Size) -> Size {
        Size::new(
            clamp(size.width, self.min_width, self.max_width),
            clamp(size.height, self.min_height, self.max_height),
        )
    }
}

#[derive(Debug)]
pub enum LayoutResult {
    Size(Size),
    RequestChild(Id, BoxConstraints),
}

/// The context given to layout methods.
pub struct LayoutCtx {
    /// Bounding box of each widget. The position is relative to the parent.
    pub geom: Vec<Bounds>,

    /// Queue of events to distribute to listeners
    pub event_q: Vec<(Id, Box<Any>)>,

    /// Which widget is currently focused, if any.
    pub focused: Option<Id>,
}

impl LayoutCtx {
    pub fn new() -> LayoutCtx {
        LayoutCtx {
            geom: Vec::new(),
            event_q: Vec::new(),
            focused: None,
        }
    }

    pub fn position_child(&mut self, child: Id, origin: Point) {
        self.geom[child].origin = origin;
    }

    pub fn get_child_size(&self, child: Id) -> Size {
        self.geom[child].size
    }
}
