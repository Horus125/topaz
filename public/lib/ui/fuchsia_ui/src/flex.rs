// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::geometry::{size_to_point, Size};
use crate::ui_state::UiState;
use crate::widget::Widget;
use crate::{BoxConstraints, LayoutResult};
use crate::{Id, LayoutCtx};

pub struct Row;
pub struct Column;

pub struct Flex {
    direction: Axis,

    // layout continuation state
    ix: usize,
    major_per_flex: f32,
    minor: f32,
}

pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    fn major(&self, coords: Size) -> f32 {
        match *self {
            Axis::Horizontal => coords.width,
            Axis::Vertical => coords.height,
        }
    }

    fn minor(&self, coords: Size) -> f32 {
        match *self {
            Axis::Horizontal => coords.height,
            Axis::Vertical => coords.width,
        }
    }

    fn pack(&self, major: f32, minor: f32) -> Size {
        match *self {
            Axis::Horizontal => Size::new(major, minor),
            Axis::Vertical => Size::new(minor, major),
        }
    }
}

impl Row {
    pub fn new() -> Flex {
        Flex {
            direction: Axis::Horizontal,

            ix: 0,
            major_per_flex: 0.0,
            minor: 0.0,
        }
    }
}

impl Column {
    pub fn new() -> Flex {
        Flex {
            direction: Axis::Vertical,

            ix: 0,
            major_per_flex: 0.0,
            minor: 0.0,
        }
    }
}

impl Flex {
    pub fn ui(self, children: &[Id], ctx: &mut UiState) -> Id {
        ctx.add(self, children)
    }
}

impl Widget for Flex {
    fn layout(
        &mut self, bc: &BoxConstraints, children: &[Id], size: Option<Size>, ctx: &mut LayoutCtx,
    ) -> LayoutResult {
        if let Some(size) = size {
            let minor = self.direction.minor(size);
            self.minor = self.minor.max(minor);
            self.ix += 1;
            if self.ix == children.len() {
                // measured all children
                let mut major = 0.0;
                for &child in children {
                    // top-align, could do center etc. based on child height
                    ctx.position_child(child, size_to_point(&self.direction.pack(major, 0.0)));
                    major += self.major_per_flex;
                }
                let max_major = self.direction.major(Size::new(bc.max_width, bc.max_height));
                return LayoutResult::Size(self.direction.pack(max_major, self.minor));
            }
        } else {
            if children.is_empty() {
                return LayoutResult::Size(Size::new(bc.min_width, bc.min_height));
            }
            self.ix = 0;
            self.minor = self.direction.minor(Size::new(bc.min_width, bc.min_height));
            let max_major = self.direction.major(Size::new(bc.max_width, bc.max_height));
            self.major_per_flex = max_major / (children.len() as f32);
        }
        let child_bc = match self.direction {
            Axis::Horizontal => BoxConstraints {
                min_width: self.major_per_flex,
                max_width: self.major_per_flex,
                min_height: bc.min_height,
                max_height: bc.max_height,
            },
            Axis::Vertical => BoxConstraints {
                min_width: bc.min_width,
                max_width: bc.max_width,
                min_height: self.major_per_flex,
                max_height: self.major_per_flex,
            },
        };
        LayoutResult::RequestChild(children[self.ix], child_bc)
    }
}
