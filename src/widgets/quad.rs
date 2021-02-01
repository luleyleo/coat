use std::panic::Location;

use druid::{kurbo::Circle, BoxConstraints, Color, Event, LifeCycle, Point, RenderContext};

use crate::{
    context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, UpdateCtx},
    cx::Cx,
    render::{Properties, RenderObject},
    tree::Children,
};

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
    cursor: Point,
}

impl Default for QuadObject {
    fn default() -> Self {
        QuadObject {
            props: Quad::new(),
            cursor: Point::ZERO,
        }
    }
}

impl RenderObject for QuadObject {
    type Props = Quad;
    type Action = ();

    fn update(&mut self, ctx: &mut UpdateCtx, props: Self::Props) {
        self.props = props;
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut Children) {
        if let Event::MouseMove(event) = event {
            self.cursor = event.pos;
            ctx.request_paint();
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        println!("Quad received lifecycle.");
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        children: &mut Children,
    ) -> druid::Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &self.props.color);
        let circle = Circle::new(self.cursor, 10.0);
        ctx.fill(circle, &Color::SILVER);
    }
}
