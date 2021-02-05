use coat::{
    app::App,
    cx::Cx,
    widgets::{padding::Padding, quad::Quad},
};
use druid::Color;

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

fn app(cx: &mut Cx) {
    Padding::new(10.0).build(cx, |cx| {
        Quad::new().color(Color::TEAL).build(cx);
    });
}
