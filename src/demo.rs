#![allow(dead_code)]

use crate::{
    app::App,
    constraints::Constraints,
    kurbo::Size,
    piet::{Color, Piet, RenderContext},
    shell::{Application, WindowBuilder},
    tree::Element,
    ui::Ui,
};

pub struct ButtonElement {
    pub color: Color,
}
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

pub fn demo_app(ui: &mut Ui) {
    button(ui, Color::RED);
}

pub fn main() {
    let application = Application::new().unwrap();

    let app = App::new(demo_app);
    let mut builder = WindowBuilder::new(application.clone());
    builder.set_handler(Box::new(app));
    builder.set_title("Coat");
    let window = builder.build().unwrap();
    window.show();

    application.run(None);
}
