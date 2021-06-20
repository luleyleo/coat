use crate::{
    constraints::Constraints,
    context::ElementCtx,
    event::Event,
    kurbo::Size,
    piet::{Piet, PietText},
    tree::{Content, Handled},
};
use std::any::Any;

pub trait Element: AsAny {
    fn paint(&mut self, element: &mut ElementCtx, piet: &mut Piet, content: &mut Content);

    fn layout(
        &mut self,
        element: &mut ElementCtx,
        constraints: &Constraints,
        content: &mut Content,
        text: &mut PietText,
    ) -> Size;

    fn event(&mut self, element: &mut ElementCtx, event: &Event, content: &mut Content) -> Handled;
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
