// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::event::HandlerCtx;
use crate::event::KeyEvent;
use crate::geometry::{new_empty_bounds, Bounds, Point, Size};
use crate::graph::Graph;
use crate::layout::{BoxConstraints, LayoutCtx, LayoutResult};
use crate::listener::ListenerCtx;
use crate::paint::PaintCtx;
use crate::widget::Widget;
use crate::Id;
use std::any::Any;
use std::collections::BTreeMap;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct UiState {
    pub listeners: BTreeMap<Id, Vec<Box<FnMut(&mut Any, ListenerCtx)>>>,

    pub command_listener: Option<Box<FnMut(u32, ListenerCtx)>>,

    /// The widget tree and associated state is split off into a separate struct
    /// so that we can use a mutable reference to it as the listener context.
    pub inner: UiInner,
}

/// The context given to listeners.
///
/// Listeners are allowed to poke widgets and mutate the graph.
pub struct UiInner {
    /// The individual widget trait objects.
    pub widgets: Vec<Box<Widget>>,

    /// Graph of widgets (actually a strict tree structure, so maybe should be renamed).
    pub graph: Graph,

    /// The state (other than widget tree) is a separate object, so that a
    /// mutable reference to it can be used as a layout context.
    pub c: LayoutCtx,
}

impl UiState {
    pub fn new() -> UiState {
        UiState {
            listeners: Default::default(),
            command_listener: None,
            inner: UiInner {
                widgets: Vec::new(),
                graph: Default::default(),
                c: LayoutCtx {
                    geom: Vec::new(),
                    event_q: Vec::new(),
                    focused: None,
                },
            },
        }
    }

    /// Add a listener that expects a specific type.
    pub fn add_listener<A, F>(&mut self, node: Id, mut f: F)
    where
        A: Any,
        F: FnMut(&mut A, ListenerCtx) + 'static,
    {
        let wrapper: Box<FnMut(&mut Any, ListenerCtx)> = Box::new(move |a, ctx| {
            if let Some(arg) = a.downcast_mut() {
                f(arg, ctx)
            } else {
                println!("type mismatch in listener arg");
            }
        });
        self.listeners
            .entry(node)
            .or_insert(Vec::new())
            .push(wrapper);
    }

    /// Set a listener for menu commands.
    pub fn set_command_listener<F>(&mut self, f: F)
    where
        F: FnMut(u32, ListenerCtx) + 'static,
    {
        self.command_listener = Some(Box::new(f));
    }

    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if let Some(id) = self.c.focused {
            let handled = {
                let mut ctx = HandlerCtx {
                    id,
                    c: &mut self.inner.c,
                };
                self.inner.widgets[id].key(event, &mut ctx)
            };
            self.dispatch_events();
            handled
        } else {
            false
        }
    }

    pub fn handle_command(&mut self, cmd: u32) {
        if let Some(ref mut listener) = self.command_listener {
            let ctx = ListenerCtx {
                id: self.inner.graph.root,
                inner: &mut self.inner,
            };
            listener(cmd, ctx);
        } else {
            println!("command received but no handler");
        }
    }

    fn dispatch_events(&mut self) {
        while !self.c.event_q.is_empty() {
            let event_q = mem::replace(&mut self.c.event_q, Vec::new());
            for (id, mut event) in event_q {
                if let Some(listeners) = self.listeners.get_mut(&id) {
                    for listener in listeners {
                        let ctx = ListenerCtx {
                            id,
                            inner: &mut self.inner,
                        };
                        listener(event.deref_mut(), ctx);
                    }
                }
            }
        }
    }
}

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

impl Deref for UiState {
    type Target = UiInner;

    fn deref(&self) -> &UiInner {
        &self.inner
    }
}

impl DerefMut for UiState {
    fn deref_mut(&mut self) -> &mut UiInner {
        &mut self.inner
    }
}

impl UiInner {
    /// Send an arbitrary payload to a widget. The type and interpretation of the
    /// payload depends on the specific target widget.
    pub fn poke<A: Any>(&mut self, node: Id, payload: &mut A) -> bool {
        let mut ctx = HandlerCtx {
            id: node,
            c: &mut self.c,
        };
        self.widgets[node].poke(payload, &mut ctx)
    }

    /// Put a widget in the graph and add its children. Returns newly allocated
    /// id for the node.
    pub fn add<W>(&mut self, widget: W, children: &[Id]) -> Id
    where
        W: Widget + 'static,
    {
        let id = self.graph.alloc_node();
        self.widgets.push(Box::new(widget));
        self.c.geom.push(new_empty_bounds());
        for &child in children {
            self.graph.append_child(id, child);
        }
        id
    }

    pub fn set_root(&mut self, root: Id) {
        self.graph.root = root;
    }

    /// Set the focused widget.
    pub fn set_focus(&mut self, node: Option<Id>) {
        self.c.focused = node;
    }

    // The following methods are really UiState methods, but don't need access to listeners
    // so are more concise to implement here.

    pub fn paint(&mut self, paint_ctx: &mut PaintCtx, root: Id) {
        // Do pre-order traversal on graph, painting each node in turn.
        //
        // Implemented as a recursion, but we could use an explicit queue instead.
        fn paint_rec(
            widgets: &mut [Box<Widget>], graph: &Graph, bounds: &[Bounds],
            paint_ctx: &mut PaintCtx, node: Id, pos: Point,
        ) {
            let g = bounds[node].translate(&pos.to_vector());
            widgets[node].paint(paint_ctx, &g);
            for child in graph.children[node].clone() {
                paint_rec(widgets, graph, bounds, paint_ctx, child, g.origin);
            }
        }

        paint_rec(
            &mut self.widgets,
            &self.graph,
            &self.c.geom,
            paint_ctx,
            root,
            Point::new(0.0, 0.0),
        );
    }

    pub fn layout(&mut self, bc: &BoxConstraints, root: Id) {
        fn layout_rec(
            widgets: &mut [Box<Widget>], ctx: &mut LayoutCtx, graph: &Graph, bc: &BoxConstraints,
            node: Id,
        ) -> Size {
            let mut size = None;
            loop {
                let layout_res = widgets[node].layout(bc, &graph.children[node], size, ctx);
                match layout_res {
                    LayoutResult::Size(size) => {
                        ctx.geom[node].size = size;
                        return size;
                    }
                    LayoutResult::RequestChild(child, child_bc) => {
                        size = Some(layout_rec(widgets, ctx, graph, &child_bc, child));
                    }
                }
            }
        }

        layout_rec(&mut self.widgets, &mut self.c, &self.graph, bc, root);
    }
}
