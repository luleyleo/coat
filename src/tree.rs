use std::{
    any::Any,
    ops::{Index, IndexMut},
};

use druid::{Cursor, Insets, Point, Rect, Region, Size, Vec2};

use crate::{
    bloom::Bloom,
    context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx},
    event::{Event, LifeCycle},
    id::ChildId,
    key::Caller,
    render::AnyRenderObject,
    BoxConstraints,
};

#[derive(Default)]
pub struct Children {
    pub(crate) states: Vec<State>,
    pub(crate) renders: Vec<Child>,
}

pub struct TreeIter<'a> {
    tree: &'a mut Children,
    state_index: usize,
    render_index: usize,
}

pub struct State {
    pub(crate) key: Caller,
    pub(crate) state: Box<dyn Any>,
    pub(crate) dead: bool,
}

pub struct Child {
    pub(crate) key: Caller,
    pub(crate) object: Box<dyn AnyRenderObject>,
    pub(crate) children: Children,
    pub(crate) state: ChildState,
    pub(crate) dead: bool,
}

pub struct ChildState {
    pub(crate) actions: Vec<Box<dyn Any>>,

    pub(crate) id: ChildId,

    /// The size of the child; this is the value returned by the child's layout
    /// method.
    pub(crate) size: Size,

    /// The origin of the child in the parent's coordinate space; together with
    /// `size` these constitute the child's layout rect.
    pub(crate) origin: Point,

    /// The origin of the parent in the window coordinate space;
    pub(crate) parent_window_origin: Point,

    /// A flag used to track and debug missing calls to set_origin.
    pub(crate) is_expecting_set_origin_call: bool,

    /// The insets applied to the layout rect to generate the paint rect.
    /// In general, these will be zero; the exception is for things like
    /// drop shadows or overflowing text.
    pub(crate) paint_insets: Insets,

    /// The offset of the baseline relative to the bottom of the widget.
    ///
    /// In general, this will be zero; the bottom of the widget will be considered
    /// the baseline. Widgets that contain text or controls that expect to be
    /// laid out alongside text can set this as appropriate.
    pub(crate) baseline_offset: f64,

    // The region that needs to be repainted, relative to the widget's bounds.
    pub(crate) invalid: Region,

    // The part of this widget that is visible on the screen is offset by this
    // much. This will be non-zero for widgets that are children of `Scroll`, or
    // similar, and it is used for propagating invalid regions.
    pub(crate) viewport_offset: Vec2,

    // TODO: consider using bitflags for the booleans.
    pub(crate) is_hot: bool,

    pub(crate) is_active: bool,

    pub(crate) needs_layout: bool,

    /// Any descendant is active.
    pub(crate) has_active: bool,

    /// In the focused path, starting from window and ending at the focused widget.
    /// Descendants of the focused widget are not in the focused path.
    pub(crate) has_focus: bool,

    /// Any descendant has requested an animation frame.
    pub(crate) request_anim: bool,

    /// Any descendant has requested update.
    pub(crate) request_update: bool,

    //pub(crate) focus_chain: Vec<WidgetId>,
    pub(crate) request_focus: Option<FocusChange>,

    pub(crate) children: Bloom<ChildId>,

    /// The cursor that was set using one of the context methods.
    pub(crate) cursor_change: CursorChange,

    /// The result of merging up children cursors. This gets cleared when merging state up (unlike
    /// cursor_change, which is persistent).
    pub(crate) cursor: Option<Cursor>,
}

/// Methods by which a widget can attempt to change focus state.
#[derive(Debug, Clone, Copy)]
pub(crate) enum FocusChange {
    /// The focused widget is giving up focus.
    Resign,
    /// A specific widget wants focus
    Focus(ChildId),
    /// Focus should pass to the next focusable widget
    Next,
    /// Focus should pass to the previous focusable widget
    Previous,
}

/// The possible cursor states for a widget.
#[derive(Clone, Debug)]
pub(crate) enum CursorChange {
    /// No cursor has been set.
    Default,
    /// Someone set a cursor, but if a child widget also set their cursor then we'll use theirs
    /// instead of ours.
    Set(Cursor),
    /// Someone set a cursor, and we'll use it regardless of what the children say.
    Override(Cursor),
}

impl Children {
    pub(crate) fn new() -> Self {
        Children::default()
    }
}

impl<'a> TreeIter<'a> {
    fn new(tree: &'a mut Children) -> Self {
        TreeIter {
            tree,
            state_index: 0,
            render_index: 0,
        }
    }
}

/// Public API for accessing children.
impl Children {
    pub fn len(&self) -> usize {
        self.renders.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<&Child> {
        self.renders.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Child> {
        self.renders.get_mut(index)
    }
}

impl Index<usize> for Children {
    type Output = Child;

    fn index(&self, index: usize) -> &Self::Output {
        &self.renders[index]
    }
}

impl IndexMut<usize> for Children {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.renders[index]
    }
}

/// Public API for child nodes.
impl Child {
    pub fn event(&mut self, ctx: &mut EventCtx, event: &Event) {
        self.object.event(ctx, event, &mut self.children)
    }

    pub fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        self.object.lifecycle(ctx, event)
    }

    pub fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        self.object.layout(ctx, bc, &mut self.children)
    }

    pub fn paint(&mut self, ctx: &mut PaintCtx) {
        self.object.paint(ctx, &mut self.children)
    }

    /// The distance from the bottom of this widget to the baseline.
    pub fn baseline_offset(&self) -> f64 {
        self.state.baseline_offset
    }
}

/// Allows iterating over a set of [`Children`].
pub struct ChildIter<'a> {
    children: &'a mut Children,
    index: usize,
}
impl<'a> Iterator for ChildIter<'a> {
    type Item = &'a mut Child;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.children.renders.get(self.index - 1).map(|node| {
            let node_p = node as *const Child as *mut Child;
            // This is save because each child can only be accessed once.
            unsafe { &mut *node_p }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.children.len(), Some(self.children.len()))
    }
}
impl<'a> IntoIterator for &'a mut Children {
    type Item = &'a mut Child;
    type IntoIter = ChildIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChildIter {
            children: self,
            index: 0,
        }
    }
}

impl ChildState {
    pub(crate) fn new(id: ChildId, size: Option<Size>) -> Self {
        ChildState {
            id,
            actions: Vec::new(),
            origin: Point::ORIGIN,
            parent_window_origin: Point::ORIGIN,
            size: size.unwrap_or_default(),
            is_expecting_set_origin_call: true,
            paint_insets: Insets::ZERO,
            invalid: Region::EMPTY,
            viewport_offset: Vec2::ZERO,
            baseline_offset: 0.0,
            is_hot: false,
            needs_layout: false,
            is_active: false,
            has_active: false,
            has_focus: false,
            request_anim: false,
            request_update: false,
            request_focus: None,
            //focus_chain: Vec::new(),
            children: Bloom::new(),
            //children_changed: false,
            //timers: HashMap::new(),
            cursor_change: CursorChange::Default,
            cursor: None,
            //sub_window_hosts: Vec::new(),
        }
    }

    // pub(crate) fn add_timer(&mut self, timer_token: TimerToken) {
    //     self.timers.insert(timer_token, self.id);
    // }

    /// Update to incorporate state changes from a child.
    ///
    /// This will also clear some requests in the child state.
    ///
    /// This method is idempotent and can be called multiple times.
    fn merge_up(&mut self, child_state: &mut ChildState) {
        let clip = self
            .layout_rect()
            .with_origin(Point::ORIGIN)
            .inset(self.paint_insets);
        let offset = child_state.layout_rect().origin().to_vec2() - child_state.viewport_offset;
        for &r in child_state.invalid.rects() {
            let r = (r + offset).intersect(clip);
            if r.area() != 0.0 {
                self.invalid.add_rect(r);
            }
        }
        // Clearing the invalid rects here is less fragile than doing it while painting. The
        // problem is that widgets (for example, Either) might choose not to paint certain
        // invisible children, and we shouldn't allow these invisible children to accumulate
        // invalid rects.
        child_state.invalid.clear();

        self.needs_layout |= child_state.needs_layout;
        self.request_anim |= child_state.request_anim;
        self.has_active |= child_state.has_active;
        self.has_focus |= child_state.has_focus;
        //self.children_changed |= child_state.children_changed;
        self.request_update |= child_state.request_update;
        self.request_focus = child_state.request_focus.take().or(self.request_focus);
        //self.timers.extend_drain(&mut child_state.timers);

        // We reset `child_state.cursor` no matter what, so that on the every pass through the tree,
        // things will be recalculated just from `cursor_change`.
        let child_cursor = child_state.take_cursor();
        if let CursorChange::Override(cursor) = &self.cursor_change {
            self.cursor = Some(cursor.clone());
        } else if child_state.has_active || child_state.is_hot {
            self.cursor = child_cursor;
        }

        if self.cursor.is_none() {
            if let CursorChange::Set(cursor) = &self.cursor_change {
                self.cursor = Some(cursor.clone());
            }
        }
    }

    /// Because of how cursor merge logic works, we need to handle the leaf case;
    /// in that case there will be nothing in the `cursor` field (as merge_up
    /// is never called) and so we need to also check the `cursor_change` field.
    fn take_cursor(&mut self) -> Option<Cursor> {
        self.cursor.take().or_else(|| self.cursor_change.cursor())
    }

    #[inline]
    pub(crate) fn size(&self) -> Size {
        self.size
    }

    /// The paint region for this widget.
    ///
    /// For more information, see [`WidgetPod::paint_rect`].
    ///
    /// [`WidgetPod::paint_rect`]: struct.WidgetPod.html#method.paint_rect
    pub(crate) fn paint_rect(&self) -> Rect {
        self.layout_rect() + self.paint_insets
    }

    pub(crate) fn layout_rect(&self) -> Rect {
        Rect::from_origin_size(self.origin, self.size)
    }

    // pub(crate) fn add_sub_window_host(&mut self, window_id: WindowId, host_id: WidgetId) {
    //     self.sub_window_hosts.push((window_id, host_id))
    // }

    pub(crate) fn window_origin(&self) -> Point {
        self.parent_window_origin + self.origin.to_vec2() - self.viewport_offset
    }
}

impl CursorChange {
    fn cursor(&self) -> Option<Cursor> {
        match self {
            CursorChange::Set(c) | CursorChange::Override(c) => Some(c.clone()),
            CursorChange::Default => None,
        }
    }
}
