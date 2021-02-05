use crate::{
    context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, UpdateCtx},
    event::{Event, LifeCycle},
    kurbo::Size,
    tree::Children,
    BoxConstraints,
};
use std::any::{type_name, Any};

pub mod prelude {
    pub use crate::context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, UpdateCtx};
    pub use crate::cx::Cx;
    pub use crate::render::{Properties, RenderObject};
    pub use crate::tree::{Child, Children};
    pub use druid::{BoxConstraints, Event, LifeCycle, RenderContext, Size};
    pub use std::panic::Location;
}

pub trait Properties {
    type Object: RenderObject;

    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub trait RenderObject {
    type Props: Properties;
    type Action;

    fn update(&mut self, ctx: &mut UpdateCtx, props: Self::Props);
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut Children);
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle);
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, children: &mut Children)
        -> Size;
    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children);
}

pub trait AnyRenderObject: Any {
    fn as_any(&mut self) -> &mut dyn Any;
    fn name(&self) -> &'static str;

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut Children);
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle);
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, children: &mut Children)
        -> Size;
    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children);
}

impl<R> AnyRenderObject for R
where
    R: RenderObject + Any,
{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn name(&self) -> &'static str {
        <R::Props as Properties>::name()
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut Children) {
        R::event(self, ctx, event, children)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        R::lifecycle(self, ctx, event)
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        children: &mut Children,
    ) -> Size {
        R::layout(self, ctx, bc, children)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children) {
        R::paint(self, ctx, children)
    }
}
