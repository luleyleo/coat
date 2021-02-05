use std::{
    any::Any,
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use druid::{
    kurbo::Shape, Affine, Cursor, Insets, InternalEvent, Point, Rect, Region, RenderContext, Size,
    Vec2,
};

use crate::{
    bloom::Bloom,
    context::{ContextState, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx},
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
    pub(crate) has_actions: bool,

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

/// [`RenderObject`] API for `Child` nodes.
impl Child {
    pub fn event(&mut self, ctx: &mut EventCtx, event: &Event) {
        if ctx.is_handled {
            // This function is called by containers to propagate an event from
            // containers to children. Non-recurse events will be invoked directly
            // from other points in the library.
            return;
        }
        let had_active = self.state.has_active;
        let rect = self.layout_rect();

        // If we need to replace either the event or its data.
        let mut modified_event = None;

        let recurse = match event {
            Event::Internal(internal) => match internal {
                InternalEvent::MouseLeave => {
                    let hot_changed = Child::set_hot_state(
                        self.object.as_mut(),
                        &mut self.state,
                        ctx.state,
                        rect,
                        None,
                    );
                    had_active || hot_changed
                }
                InternalEvent::TargetedCommand(cmd) => false,
                InternalEvent::RouteTimer(token, widget_id) => false,
            },
            Event::WindowConnected => true,
            Event::WindowSize(_) => {
                self.state.needs_layout = true;
                ctx.is_root
            }
            Event::MouseDown(mouse_event) => {
                Child::set_hot_state(
                    self.object.as_mut(),
                    &mut self.state,
                    ctx.state,
                    rect,
                    Some(mouse_event.pos),
                );
                if had_active || self.state.is_hot {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos -= rect.origin().to_vec2();
                    modified_event = Some(Event::MouseDown(mouse_event));
                    true
                } else {
                    false
                }
            }
            Event::MouseUp(mouse_event) => {
                Child::set_hot_state(
                    self.object.as_mut(),
                    &mut self.state,
                    ctx.state,
                    rect,
                    Some(mouse_event.pos),
                );
                if had_active || self.state.is_hot {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos -= rect.origin().to_vec2();
                    modified_event = Some(Event::MouseUp(mouse_event));
                    true
                } else {
                    false
                }
            }
            Event::MouseMove(mouse_event) => {
                let hot_changed = Child::set_hot_state(
                    self.object.as_mut(),
                    &mut self.state,
                    ctx.state,
                    rect,
                    Some(mouse_event.pos),
                );
                // MouseMove is recursed even if the widget is not active and not hot,
                // but was hot previously. This is to allow the widget to respond to the movement,
                // e.g. drag functionality where the widget wants to follow the mouse.
                if had_active || self.state.is_hot || hot_changed {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos -= rect.origin().to_vec2();
                    modified_event = Some(Event::MouseMove(mouse_event));
                    true
                } else {
                    false
                }
            }
            Event::Wheel(mouse_event) => {
                Child::set_hot_state(
                    self.object.as_mut(),
                    &mut self.state,
                    ctx.state,
                    rect,
                    Some(mouse_event.pos),
                );
                if had_active || self.state.is_hot {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos -= rect.origin().to_vec2();
                    modified_event = Some(Event::Wheel(mouse_event));
                    true
                } else {
                    false
                }
            }
            Event::AnimFrame(_) => {
                let r = self.state.request_anim;
                self.state.request_anim = false;
                r
            }
            Event::KeyDown(_) => self.state.has_focus,
            Event::KeyUp(_) => self.state.has_focus,
            Event::Paste(_) => self.state.has_focus,
            Event::Zoom(_) => had_active || self.state.is_hot,
            Event::Timer(_) => false, // This event was targeted only to our parent
            Event::Command(_) => true,
            Event::Notification(_) => false,
        };

        if recurse {
            let mut inner_ctx = EventCtx {
                state: ctx.state,
                child_state: &mut self.state,
                is_handled: false,
                is_root: false,
            };
            let inner_event = modified_event.as_ref().unwrap_or(event);
            inner_ctx.child_state.has_active = false;

            self.object
                .event(&mut inner_ctx, &inner_event, &mut self.children);

            inner_ctx.child_state.has_active |= inner_ctx.child_state.is_active;
            ctx.is_handled |= inner_ctx.is_handled;
        }

        ctx.child_state.merge_up(&mut self.state);
    }

    pub fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        let mut child_ctx = LifeCycleCtx {
            state: ctx.state,
            child_state: &mut self.state,
        };

        self.object.lifecycle(&mut child_ctx, event);

        ctx.child_state.merge_up(&mut self.state);
    }

    pub fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        self.state.needs_layout = false;
        self.state.is_expecting_set_origin_call = true;

        let child_mouse_pos = match ctx.mouse_pos {
            Some(pos) => Some(pos - self.layout_rect().origin().to_vec2() + self.viewport_offset()),
            None => None,
        };
        let prev_size = self.state.size;

        let mut child_ctx = LayoutCtx {
            state: ctx.state,
            child_state: &mut self.state,
            mouse_pos: child_mouse_pos,
        };

        let new_size = self.object.layout(&mut child_ctx, bc, &mut self.children);
        if new_size != prev_size {
            let mut child_ctx = LifeCycleCtx {
                child_state: child_ctx.child_state,
                state: child_ctx.state,
            };
            let size_event = LifeCycle::Size(new_size);
            self.object.lifecycle(&mut child_ctx, &size_event);
        }

        ctx.child_state.merge_up(&mut child_ctx.child_state);
        self.state.size = new_size;
        self.log_layout_issues(new_size);

        new_size
    }

    fn log_layout_issues(&self, size: Size) {
        if size.width.is_infinite() {
            let name = self.object.name();
            log::warn!("Widget `{}` has an infinite width.", name);
        }
        if size.height.is_infinite() {
            let name = self.object.name();
            log::warn!("Widget `{}` has an infinite height.", name);
        }
    }

    pub fn paint(&mut self, ctx: &mut PaintCtx) {
        ctx.with_save(|ctx| {
            let layout_origin = self.layout_rect().origin().to_vec2();
            ctx.transform(Affine::translate(layout_origin));
            let mut visible = ctx.region().clone();
            visible.intersect_with(self.state.paint_rect());
            visible -= layout_origin;
            ctx.with_child_ctx(visible, |ctx| self.paint_raw(ctx));
        });
    }

    /// Paint a child widget.
    ///
    /// Generally called by container widgets as part of their [`Widget::paint`]
    /// method.
    ///
    /// Note that this method does not apply the offset of the layout rect.
    /// If that is desired, use [`paint`] instead.
    ///
    /// [`layout`]: trait.Widget.html#tymethod.layout
    /// [`Widget::paint`]: trait.Widget.html#tymethod.paint
    /// [`paint`]: #method.paint
    pub fn paint_raw(&mut self, ctx: &mut PaintCtx) {
        // we need to do this before we borrow from self
        // if env.get(Env::DEBUG_WIDGET_ID) {
        //     self.make_widget_id_layout_if_needed(self.state.id, ctx, env);
        // }

        let mut inner_ctx = PaintCtx {
            render_ctx: ctx.render_ctx,
            state: ctx.state,
            z_ops: Vec::new(),
            region: ctx.region.clone(),
            child_state: &self.state,
            depth: ctx.depth,
        };
        self.object.paint(&mut inner_ctx, &mut self.children);

        // let debug_ids = inner_ctx.is_hot() && env.get(Env::DEBUG_WIDGET_ID);
        // if debug_ids {
        //     // this also draws layout bounds
        //     self.debug_paint_widget_ids(&mut inner_ctx, env);
        // }

        // if !debug_ids && env.get(Env::DEBUG_PAINT) {
        //     self.debug_paint_layout_bounds(&mut inner_ctx, env);
        // }

        ctx.z_ops.append(&mut inner_ctx.z_ops);
    }
}

/// Public API for child nodes.
impl Child {
    /// Query the "active" state of the widget.
    pub fn is_active(&self) -> bool {
        self.state.is_active
    }

    /// Returns `true` if any descendant is active.
    pub fn has_active(&self) -> bool {
        self.state.has_active
    }

    /// Query the "hot" state of the widget.
    ///
    /// See [`EventCtx::is_hot`](struct.EventCtx.html#method.is_hot) for
    /// additional information.
    pub fn is_hot(&self) -> bool {
        self.state.is_hot
    }

    /// Set the origin of this widget, in the parent's coordinate space.
    ///
    /// A container widget should call the [`Widget::layout`] method on its children in
    /// its own [`Widget::layout`] implementation, and then call `set_origin` to
    /// position those children.
    ///
    /// The child will receive the [`LifeCycle::Size`] event informing them of the final [`Size`].
    ///
    /// [`Widget::layout`]: trait.Widget.html#tymethod.layout
    /// [`Rect`]: struct.Rect.html
    /// [`Size`]: struct.Size.html
    /// [`LifeCycle::Size`]: enum.LifeCycle.html#variant.Size
    pub fn set_origin(&mut self, ctx: &mut LayoutCtx, origin: Point) {
        self.state.origin = origin;
        self.state.is_expecting_set_origin_call = false;
        let layout_rect = self.layout_rect();

        // if the widget has moved, it may have moved under the mouse, in which
        // case we need to handle that.
        if Child::set_hot_state(
            self.object.as_mut(),
            &mut self.state,
            ctx.state,
            layout_rect,
            ctx.mouse_pos,
        ) {
            ctx.child_state.merge_up(&mut self.state);
        }
    }

    /// Returns the layout [`Rect`].
    ///
    /// This will be a [`Rect`] with a [`Size`] determined by the child's [`layout`]
    /// method, and the origin that was set by [`set_origin`].
    ///
    /// [`Rect`]: struct.Rect.html
    /// [`Size`]: struct.Size.html
    /// [`layout`]: trait.Widget.html#tymethod.layout
    /// [`set_origin`]: WidgetPod::set_origin
    pub fn layout_rect(&self) -> Rect {
        self.state.layout_rect()
    }

    /// Set the viewport offset.
    ///
    /// This is relevant only for children of a scroll view (or similar). It must
    /// be set by the parent widget whenever it modifies the position of its child
    /// while painting it and propagating events. As a rule of thumb, you need this
    /// if and only if you `Affine::translate` the paint context before painting
    /// your child. For an example, see the implentation of [`Scroll`].
    ///
    /// [`Scroll`]: widget/struct.Scroll.html
    pub fn set_viewport_offset(&mut self, offset: Vec2) {
        if offset != self.state.viewport_offset {
            // We need the parent_window_origin recalculated.
            // It should be possible to just trigger the InternalLifeCycle::ParentWindowOrigin here,
            // instead of full layout. Would need more management in WidgetState.
            self.state.needs_layout = true;
        }
        self.state.viewport_offset = offset;
    }

    /// The viewport offset.
    ///
    /// This will be the same value as set by [`set_viewport_offset`].
    ///
    /// [`set_viewport_offset`]: #method.viewport_offset
    pub fn viewport_offset(&self) -> Vec2 {
        self.state.viewport_offset
    }

    /// Get the widget's paint [`Rect`].
    ///
    /// This is the [`Rect`] that widget has indicated it needs to paint in.
    /// This is the same as the [`layout_rect`] with the [`paint_insets`] applied;
    /// in the general case it is the same as the [`layout_rect`].
    ///
    /// [`layout_rect`]: #method.layout_rect
    /// [`Rect`]: struct.Rect.html
    /// [`paint_insets`]: #method.paint_insets
    pub fn paint_rect(&self) -> Rect {
        self.state.paint_rect()
    }

    /// Return the paint [`Insets`] for this widget.
    ///
    /// If these [`Insets`] are nonzero, they describe the area beyond a widget's
    /// layout rect where it needs to paint.
    ///
    /// These are generally zero; exceptions are widgets that do things like
    /// paint a drop shadow.
    ///
    /// A widget can set its insets by calling [`set_paint_insets`] during its
    /// [`layout`] method.
    ///
    /// [`Insets`]: struct.Insets.html
    /// [`set_paint_insets`]: struct.LayoutCtx.html#method.set_paint_insets
    /// [`layout`]: trait.Widget.html#tymethod.layout
    pub fn paint_insets(&self) -> Insets {
        self.state.paint_insets
    }

    /// Given a parents layout size, determine the appropriate paint `Insets`
    /// for the parent.
    ///
    /// This is a convenience method to be used from the [`layout`] method
    /// of a `Widget` that manages a child; it allows the parent to correctly
    /// propogate a child's desired paint rect, if it extends beyond the bounds
    /// of the parent's layout rect.
    ///
    /// [`layout`]: trait.Widget.html#tymethod.layout
    /// [`Insets`]: struct.Insets.html
    pub fn compute_parent_paint_insets(&self, parent_size: Size) -> Insets {
        let parent_bounds = Rect::ZERO.with_size(parent_size);
        let union_pant_rect = self.paint_rect().union(parent_bounds);
        union_pant_rect - parent_bounds
    }

    /// The distance from the bottom of this widget to the baseline.
    pub fn baseline_offset(&self) -> f64 {
        self.state.baseline_offset
    }

    /// Determines if the provided `mouse_pos` is inside `rect`
    /// and if so updates the hot state and sends `LifeCycle::HotChanged`.
    ///
    /// Returns `true` if the hot state changed.
    ///
    /// The provided `child_state` should be merged up if this returns `true`.
    fn set_hot_state(
        child: &mut dyn AnyRenderObject,
        child_state: &mut ChildState,
        state: &mut ContextState,
        rect: Rect,
        mouse_pos: Option<Point>,
    ) -> bool {
        let had_hot = child_state.is_hot;
        child_state.is_hot = match mouse_pos {
            Some(pos) => rect.winding(pos) != 0,
            None => false,
        };
        if had_hot != child_state.is_hot {
            let hot_changed_event = LifeCycle::HotChanged(child_state.is_hot);
            let mut child_ctx = LifeCycleCtx { state, child_state };
            child.lifecycle(&mut child_ctx, &hot_changed_event);
            // if hot changes and we're showing widget ids, always repaint
            // if env.get(Env::DEBUG_WIDGET_ID) {
            //     child_ctx.request_paint();
            // }
            return true;
        }
        false
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
            has_actions: false,
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

        self.has_actions = !self.actions.is_empty() || child_state.has_actions;

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
