#![allow(dead_code)]

use crate::{
    app::App,
    elements::{button, column},
    piet::Color,
    shell::{Application, WindowBuilder},
    ui::Ui,
};

pub fn demo_app(ui: &mut Ui) {
    column(ui, |ui| {
        button(ui, Color::RED);
        button(ui, Color::GREEN);
    });
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
