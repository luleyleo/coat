use crate::{
    context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx},
    event::{Event, LifeCycle},
    kurbo::Size,
    BoxConstraints,
};
use std::any::Any;

pub struct Child {
    inner: Box<dyn AnyRenderObject>,
}

pub trait RenderObject<P> {
    fn mutate(&mut self, props: &P);
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut [Child]);
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle);
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, children: &mut [Child]) -> Size;
    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut [Child]);
}

pub trait AnyRenderObject: Any {
    fn mutate(&mut self, props: &dyn Any);
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut [Child]);
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle);
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, children: &mut [Child]) -> Size;
    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut [Child]);
}
