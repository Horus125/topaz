// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate euclid;
extern crate failure;

type Id = usize;

mod color;
mod event;
mod flex;
mod geometry;
mod graph;
mod layout;
mod listener;
mod padding;
mod paint;
pub mod ui_state;
mod widget;

pub use crate::flex::{Column, Row};
pub use crate::geometry::Size;
pub use crate::layout::{BoxConstraints, LayoutCtx, LayoutResult};
pub use crate::padding::Padding;
pub use crate::paint::PaintCtx;
pub use crate::ui_state::UiState;
pub use crate::widget::Box;
