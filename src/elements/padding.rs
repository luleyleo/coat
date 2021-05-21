use crate::{
    constraints::Constraints,
    event::Event,
    kurbo::{Insets, Point, Size},
    piet::{Piet, PietText},
    tree::{Content, Element},
    ui::Ui,
};

#[derive(Default)]
pub struct Padding {
    insets: Insets,
}

impl Element for Padding {
    fn paint(&mut self, piet: &mut Piet, _size: Size, content: &mut Content) {
        for mut child in content.iter_mut() {
            child.paint(piet);
        }
    }

    fn layout(
        &mut self,
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

    fn event(&mut self, event: &Event, handled: &mut bool, content: &mut Content) {
        for mut child in content.iter_mut() {
            child.event(event, handled);
        }
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
