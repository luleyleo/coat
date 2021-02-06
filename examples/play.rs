use coat::{
    app::App,
    cx::Cx,
    widgets::{button::Button, padding::Padding},
};

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

fn app(cx: &mut Cx) {
    Padding::new(100.0).build(cx, |cx| {
        if Button::new().labeled(cx, "Hello world!") {
            println!("The Button has been clicked!");
        }
    });
}
