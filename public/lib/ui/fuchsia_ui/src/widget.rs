// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::color::Color;
use crate::event::{HandlerCtx, KeyEvent, MouseEvent};
use crate::geometry::{Bounds, Point, Size};
use crate::layout::{BoxConstraints, LayoutCtx, LayoutResult};
use crate::paint::PaintCtx;
use crate::ui_state::UiState;
use crate::Id;
use std::any::Any;

/// The trait implemented by all widgets.
pub trait Widget {
    /// Paint the widget's appearance into the paint context.
    ///
    /// The implementer is responsible for translating the coordinates as
    /// specified in the geometry.
    #[allow(unused)]
    fn paint(&mut self, paint_ctx: &mut PaintCtx, bounds: &Bounds) {}

    /// Participate in the layout protocol.
    ///
    /// `size` is the size of the child previously requested by a RequestChild return.
    ///
    /// The default implementation is suitable for widgets with a single child, and
    /// just forwards the layout unmodified.
    fn layout(
        &mut self, bc: &BoxConstraints, children: &[Id], size: Option<Size>, ctx: &mut LayoutCtx,
    ) -> LayoutResult {
        if let Some(size) = size {
            // Maybe this is not necessary, rely on default value.
            ctx.position_child(children[0], Point::new(0.0, 0.0));
            LayoutResult::Size(size)
        } else {
            LayoutResult::RequestChild(children[0], *bc)
        }
    }

    /// Sent to the widget on mouse event.
    ///
    /// Mouse events are propagated in a post-order traversal of the widget tree,
    /// culled by geometry. Propagation stops as soon as the event is handled.
    #[allow(unused)]
    fn mouse(&mut self, event: &MouseEvent, ctx: &mut HandlerCtx) -> bool {
        false
    }

    /// An "escape hatch" of sorts for accessing widget state beyond the widget
    /// methods. Returns true if it is handled.
    #[allow(unused)]
    fn poke(&mut self, payload: &mut Any, ctx: &mut HandlerCtx) -> bool {
        false
    }

    /// Sent to the widget on key event.
    ///
    /// Key events are only sent to the focused widget.
    ///
    /// Note that keys that are interpreted as characters are sent twice, first
    /// as a `Vkey`, then as a `Char`.
    ///
    /// This is a fairly thin wrapper over WM messages. Keyboard input will be
    /// changing quite a bit when IME is implemented.
    ///
    /// Returns true if the event is handled.
    #[allow(unused)]
    fn key(&mut self, event: &KeyEvent, ctx: &mut HandlerCtx) -> bool {
        false
    }
}

pub struct Box {}

impl Box {
    pub fn new() -> Box {
        Box {}
    }

    pub fn ui(self, ui_state: &mut UiState) -> Id {
        ui_state.add(self, &[])
    }
}

impl Widget for Box {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, bounds: &Bounds) {
        paint_ctx.fill_rect(&bounds, &Color::new());
    }

    fn layout(
        &mut self, bc: &BoxConstraints, children: &[Id], size: Option<Size>, ctx: &mut LayoutCtx,
    ) -> LayoutResult {
        let layout_result = if let Some(size) = size {
            LayoutResult::Size(size)
        } else {
            LayoutResult::Size(Size::new(bc.max_width, bc.max_height))
        };
        layout_result
    }
}
