use shell::{
    kurbo::{Point, Size},
    piet::Piet,
};

use crate::{
    constraints::Constraints,
    tree::{Content, Element},
    ui::Ui,
};

pub struct ColumnElement;

impl Element for ColumnElement {
    fn paint(&mut self, piet: &mut Piet, _size: Size, content: &mut Content) {
        for mut child in content.iter_mut() {
            child.paint(piet);
        }
    }

    fn layout(&self, constraints: &Constraints, content: &mut Content) -> Size {
        let height_per_child = constraints.max.height / content.len() as f64;
        let child_constraints = constraints.with_max_height(height_per_child);
        for (index, mut child) in content.iter_mut().enumerate() {
            child.layout(&child_constraints);
            child.set_origin(Point::new(0.0, height_per_child * index as f64));
        }
        constraints.max
    }
}

#[track_caller]
pub fn column(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let location = std::panic::Location::caller();
    ui.add(location, ColumnElement, content);
}
