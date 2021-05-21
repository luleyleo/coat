use crate::{
    constraints::Constraints,
    event::Event,
    kurbo::{Point, Size},
    piet::{Piet, PietText},
    tree::{Content, Element},
    ui::Ui,
};

#[derive(Default)]
pub struct ColumnElement;

impl Element for ColumnElement {
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
        let height_per_child = constraints.max.height / content.len() as f64;
        let child_constraints = constraints.with_max_height(height_per_child);
        for (index, mut child) in content.iter_mut().enumerate() {
            child.layout(&child_constraints, text);
            child.set_origin(Point::new(0.0, height_per_child * index as f64));
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
pub fn column(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let location = std::panic::Location::caller();
    ui.add(location, |_: &mut ColumnElement| {}, content);
}
