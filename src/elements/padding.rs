use crate::{
    constraints::Constraints,
    context::ElementCtx,
    event::Event,
    kurbo::{Insets, Point, Size},
    piet::{Piet, PietText},
    tree::{Content, Element, Handled},
    ui::Ui,
};

#[derive(Default)]
pub struct Padding {
    insets: Insets,
}

impl Element for Padding {
    fn paint(&mut self, _element: &mut ElementCtx, piet: &mut Piet, content: &mut Content) {
        for mut child in content.iter_mut() {
            child.paint(piet);
        }
    }

    fn layout(
        &mut self,
        _element: &mut ElementCtx,
        constraints: &Constraints,
        content: &mut Content,
        text: &mut PietText,
    ) -> Size {
        let child_constraints = constraints.shrink(self.insets.size());
        let offset = Point::new(self.insets.x0, self.insets.y0);
        for mut child in content.iter_mut() {
            child.layout(&child_constraints, text);
            child.set_origin(offset);
        }
        constraints.max
    }

    fn event(
        &mut self,
        _element: &mut ElementCtx,
        event: &Event,
        content: &mut Content,
    ) -> Handled {
        for mut child in content.iter_mut() {
            if child.event(event).handled() {
                return Handled(true);
            }
        }
        Handled(false)
    }
}

#[track_caller]
pub fn padding(ui: &mut Ui, insets: impl Into<Insets>, content: impl FnOnce(&mut Ui)) {
    let location = std::panic::Location::caller();
    ui.add(
        location,
        |padding: &mut Padding| padding.insets = insets.into(),
        content,
    );
}
