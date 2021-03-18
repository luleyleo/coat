
use crate::id::{ChildId, WindowId};
use crate::kurbo::Point;
use crate::object::prelude::*;

pub enum WindowEvent {
    Noop,
    Close,
    RequestFocus,
}

#[derive(PartialEq)]
pub struct Window {
    id: WindowId,
    title: String,
    initial_size: Size,
}

impl Properties for Window {
    type Object = WindowObject;
}

pub struct WindowObject {
    props: Window,
    focus_widget: Option<ChildId>,
    mouse_pos: Option<Point>,
}

impl RenderObject<Window> for WindowObject {
    type Action = ();

    fn create(props: Window) -> Self {
        todo!()
    }

    fn update(&mut self, ctx: &mut UpdateCtx, props: Window) -> Self::Action {
        todo!()
    }
}

impl RenderObjectInterface for WindowObject {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut Children) {
        todo!()
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        todo!()
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, children: &mut Children)
        -> Size {
        todo!()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children) {
        todo!()
    }
}
