// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::ui_state::UiInner;
use crate::Id;

pub struct ListenerCtx<'a> {
    pub id: Id,
    pub inner: &'a mut UiInner,
}
