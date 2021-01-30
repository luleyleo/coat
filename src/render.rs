use crate::{
    context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, UpdateCtx},
    event::{Event, LifeCycle},
    kurbo::Size,
    tree::{RenderNode, Tree},
    BoxConstraints,
};
use std::any::Any;

pub struct Children<'a> {
    tree: &'a mut Tree,
}

pub struct Child<'a> {
    object: &'a mut dyn AnyRenderObject,
    actions: &'a mut Vec<Box<dyn Any>>,
    children: Children<'a>,
}

pub trait Properties {
    type Object: RenderObject;
    type Action;
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

impl<'a> Children<'a> {
    fn len(&self) -> usize {
        self.tree.renders.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, index: usize) -> Option<&dyn AnyRenderObject> {
        self.tree
            .renders
            .get(index)
            .map(|node| node.object.as_ref())
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut dyn AnyRenderObject> {
        self.tree
            .renders
            .get_mut(index)
            .map(|node| node.object.as_mut())
    }
}

impl<'a> Child<'a> {
    pub fn event(&mut self, ctx: &mut EventCtx, event: &Event) {
        self.object.event(ctx, event, &mut self.children)
    }

    pub fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        self.object.lifecycle(ctx, event)
    }

    pub fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        children: &mut Children,
    ) -> Size {
        self.object.layout(ctx, bc, &mut self.children)
    }

    pub fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children) {
        self.object.paint(ctx, &mut self.children)
    }
}

pub struct ChildIter<'a, 'b> {
    children: &'a mut Children<'b>,
    index: usize,
}
impl<'a, 'b> Iterator for ChildIter<'a, 'b> {
    type Item = Child<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.children.tree.renders.get(self.index - 1).map(|node| {
            let node_p = node as *const RenderNode as *mut RenderNode;
            // This is save because each child can only be accessed once.
            let node = unsafe { &mut *node_p };
            Child {
                object: node.object.as_mut(),
                actions: &mut node.state.actions,
                children: Children {
                    tree: &mut node.children,
                },
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.children.len(), Some(self.children.len()))
    }
}
impl<'a, 'b> IntoIterator for &'a mut Children<'b> {
    type Item = Child<'b>;
    type IntoIter = ChildIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        ChildIter {
            children: self,
            index: 0,
        }
    }
}

pub trait AnyRenderObject: Any {
    fn as_any(&mut self) -> &mut dyn Any;

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
