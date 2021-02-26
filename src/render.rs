use crate::{
    context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, UpdateCtx},
    event::{Event, LifeCycle},
    kurbo::Size,
    tree::Children,
    BoxConstraints,
};
use std::any::Any;

pub mod prelude {
    pub use crate::{
        context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, UpdateCtx},
        event::{Event, LifeCycle},
        kurbo::Size,
        piet::RenderContext,
        render::{Properties, RenderObject, RenderObjectInterface},
        tree::{Child, Children},
        ui::Ui,
        BoxConstraints,
    };
    pub use std::panic::Location;
}

pub trait Properties: Sized {
    type Object: RenderObject<Self>;
}

pub trait RenderObject<Props>: RenderObjectInterface {
    type Action: Any;

    fn create(props: Props) -> Self;
    fn update(&mut self, ctx: &mut UpdateCtx, props: Props);
}

pub trait RenderObjectInterface {
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
    R: RenderObjectInterface + Any,
{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn name(&self) -> &'static str {
        "" //<R::Props<'_> as Properties>::name()
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
