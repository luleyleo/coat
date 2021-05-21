#![allow(dead_code)]

use coat::{
    app::App,
    elements::{button, column, padding},
    shell::{Application, WindowBuilder},
    ui::Ui,
};

pub fn demo_app(ui: &mut Ui) {
    column(ui, |ui| {
        padding(ui, 10.0, |ui| button(ui, "Hello"));
        padding(ui, 10.0, |ui| button(ui, "World"));
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
