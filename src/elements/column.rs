use crate::{
    constraints::Constraints,
    context::ElementCtx,
    event::Event,
    kurbo::{Point, Size},
    piet::{Piet, PietText},
    tree::{Content, Element, Handled},
    ui::Ui,
};

#[derive(Default)]
pub struct ColumnElement;

impl Element for ColumnElement {
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
        let height_per_child = constraints.max.height / content.len() as f64;
        let child_constraints = constraints.with_max_height(height_per_child);
        for (index, mut child) in content.iter_mut().enumerate() {
            child.layout(&child_constraints, text);
            child.set_origin(Point::new(0.0, height_per_child * index as f64));
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
pub fn column(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let location = std::panic::Location::caller();
    ui.add(location, |_: &mut ColumnElement| {}, content);
}
