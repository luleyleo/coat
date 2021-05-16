use crate::{
    constraints::Constraints,
    kurbo::Size,
    piet::{Color, Piet, RenderContext},
    tree::Element,
    ui::Ui,
};

pub struct ButtonElement {
    pub color: Color,
}

#[cfg(test)]
impl ButtonElement {
    pub fn new(color: Color) -> Self {
        ButtonElement { color }
    }
}

impl Element for ButtonElement {
    fn paint(&mut self, piet: &mut Piet, size: Size) {
        piet.fill(&size.to_rect(), &self.color);
    }

    fn layout(&self, constraints: &Constraints) -> Size {
        constraints.max
    }
}

#[track_caller]
pub fn button(ui: &mut Ui, color: Color) {
    let location = std::panic::Location::caller();
    ui.add(location, ButtonElement { color }, |_| {});
}
