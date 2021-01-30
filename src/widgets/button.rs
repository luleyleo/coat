use std::panic::Location;

use crate::{
    context::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, UpdateCtx},
    cx::Cx,
    event::{Event, LifeCycle},
    kurbo::Size,
    render::{Children, Properties, RenderObject},
    BoxConstraints,
};
use druid::MouseButton;

#[derive(Default)]
struct TextLayout;

#[derive(Default, PartialEq, Eq)]
pub struct Button {
    label: String,
    disabled: bool,
}

impl Properties for Button {
    type Object = ButtonObject;
    type Action = bool;
}

impl Button {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[track_caller]
    pub fn build(self, cx: &mut Cx) -> bool {
        let caller = Location::caller().into();
        cx.render_object::<ButtonObject>(caller, self).is_some()
    }
}

pub enum ButtonAction {
    Clicked,
}

#[derive(Default)]
pub struct ButtonObject {
    props: Button,
    layout: TextLayout,
}

impl RenderObject for ButtonObject {
    type Props = Button;
    type Action = ButtonAction;

    fn update(&mut self, ctx: &mut UpdateCtx, props: Button) {
        if self.props != props {
            //ctx.request_layout();
            self.props = props;
        }
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, children: &mut Children) {
        match event {
            Event::MouseDown(mouse_event) => {
                if mouse_event.button == MouseButton::Left {
                    //ctx.set_active(true);
                    //ctx.request_paint();
                }
            }
            Event::MouseUp(mouse_event) => {
                // if ctx.is_active() && mouse_event.button == MouseButton::Left {
                //     ctx.set_active(false);
                //     if ctx.is_hot() {
                //         ctx.submit_action(ButtonAction::Clicked);
                //         ctx.set_handled();
                //     }
                //     ctx.request_paint();
                // }
            }
            _ => {}
        }

        for mut child in children {
            child.event(ctx, event);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        todo!()
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        children: &mut Children,
    ) -> Size {
        todo!()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children) {
        todo!()
    }
}
