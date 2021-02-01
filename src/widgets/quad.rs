use std::panic::Location;

use druid::{Color, RenderContext};

use crate::{cx::Cx, render::{Properties, RenderObject}};


pub struct Quad {
    color: Color,
}

impl Quad {
    pub fn new() -> Self {
        Quad {
            color: druid::Color::RED,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    #[track_caller]
    pub fn build(self, cx: &mut Cx) {
        let caller = Location::caller().into();
        cx.render_object::<QuadObject>(caller, self);
    }
}

impl Properties for Quad {
    type Object = QuadObject;
    type Action = ();
}

pub struct QuadObject {
    props: Quad,
}

impl Default for QuadObject {
    fn default() -> Self {
        QuadObject {
            props: Quad::new()
        }
    }
}

impl RenderObject for QuadObject {
    type Props = Quad;
    type Action = ();

    fn update(&mut self, ctx: &mut crate::context::UpdateCtx, props: Self::Props) {
        println!("Quad updated.");
    }

    fn event(&mut self, ctx: &mut crate::context::EventCtx, event: &druid::Event, children: &mut crate::tree::Children) {
        println!("Quad received event.");
    }

    fn lifecycle(&mut self, ctx: &mut crate::context::LifeCycleCtx, event: &druid::LifeCycle) {
        println!("Quad received lifecycle.");
    }

    fn layout(&mut self, ctx: &mut crate::context::LayoutCtx, bc: &druid::BoxConstraints, children: &mut crate::tree::Children)
        -> druid::Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut crate::context::PaintCtx, children: &mut crate::tree::Children) {
        let size = ctx.size();
        println!("Quad painted with {:?}.", size);
        let rect = size.to_rect();
        ctx.fill(rect, &self.props.color);
    }
}
