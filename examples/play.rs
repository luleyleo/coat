use coat::{app::App, cx::Cx, widgets::{button::Button, quad::Quad}};
use druid::Color;

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

fn app(cx: &mut Cx) {
    println!("App is running!");
    Quad::new().color(Color::TEAL).build(cx);
}
