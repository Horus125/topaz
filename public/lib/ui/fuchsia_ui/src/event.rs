// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::layout::LayoutCtx;
use crate::ui_state::UiState;
use crate::Id;

/// An indicator of which mouse button was pressed.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Middle mouse button.
    Middle,
    /// Right mouse button.
    Right,
    /// First X button.
    X1,
    /// Second X button.
    X2,
}

/// An indicator of the state change of a mouse button.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MouseType {
    /// Mouse down event.
    Down,
    /// Note: DoubleClick is currently disabled, as we don't use the
    /// Windows processing.
    DoubleClick,
    /// Mouse up event.
    Up,
}

pub struct MouseEvent {
    /// X coordinate in px units, relative to top left of widget.
    pub x: f32,
    /// Y coordinate in px units, relative to top left of widget.
    pub y: f32,
    /// The modifiers, which have the same interpretation as the raw WM message.
    ///
    /// TODO: rationalize this with mouse mods.
    pub mods: u32,
    /// Which mouse button was pressed.
    pub which: MouseButton,
    /// Count of multiple clicks, is 0 for mouse up event.
    pub count: u32,
}

#[derive(Clone)]
pub struct KeyEvent {
    pub key: KeyVariant,
    /// The modifiers, a combinations of `M_ALT`, `M_CTRL`, `M_SHIFT`.
    pub mods: u32,
}

#[derive(Clone)]
pub enum KeyVariant {
    /// A virtual-key code, same as WM_KEYDOWN message.
    Vkey(i32),
    /// A Unicode character.
    Char(char),
}

/// Context given to handlers.
pub struct HandlerCtx<'a> {
    /// The id of the node sending the event
    pub id: Id,

    pub c: &'a mut LayoutCtx,
}

pub struct ListenerCtx<'a> {
    pub id: Id,

    pub ui_state: &'a mut UiState,
}
