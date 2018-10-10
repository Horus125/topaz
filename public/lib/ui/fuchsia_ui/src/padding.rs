// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::geometry::{Point, Size};
use crate::widget::Widget;
use crate::{BoxConstraints, LayoutResult};
use crate::{Id, LayoutCtx, UiState};

/// A padding widget. Is expected to have exactly one child.
pub struct Padding {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Padding {
    /// Create widget with uniform padding.
    pub fn uniform(padding: f32) -> Padding {
        Padding {
            left: padding,
            right: padding,
            top: padding,
            bottom: padding,
        }
    }

    pub fn ui(self, child: Id, ctx: &mut UiState) -> Id {
        ctx.add(self, &[child])
    }
}

impl Widget for Padding {
    fn layout(
        &mut self, bc: &BoxConstraints, children: &[Id], size: Option<Size>, ctx: &mut LayoutCtx,
    ) -> LayoutResult {
        if let Some(size) = size {
            ctx.position_child(children[0], Point::new(self.left, self.top));
            LayoutResult::Size(Size::new(
                size.width + self.left + self.right,
                size.height + self.top + self.bottom,
            ))
        } else {
            let child_bc = BoxConstraints {
                min_width: bc.min_width - (self.left + self.right),
                max_width: bc.max_width - (self.left + self.right),
                min_height: bc.min_height - (self.top + self.bottom),
                max_height: bc.max_height - (self.top + self.bottom),
            };
            LayoutResult::RequestChild(children[0], child_bc)
        }
    }
}
