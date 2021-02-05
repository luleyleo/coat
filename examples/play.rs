use coat::{
    app::App,
    cx::Cx,
    widgets::{label::Label, padding::Padding, quad::Quad},
};
use druid::Color;

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

fn app(cx: &mut Cx) {
    Padding::new(10.0).build(cx, |cx| {
        Label::new("Hello world!").build(cx);
    });
}
