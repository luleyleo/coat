use coat::{
    app::App,
    cx::Cx,
    widgets::{button::Button, label::Label, padding::Padding},
};

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

fn app(cx: &mut Cx) {
    Padding::new(100.0).build(cx, |cx| {
        let clicked = Button::new().custom(cx, |cx| {
            Label::new("Hello world!").build(cx);
        });
        if clicked {
            println!("The Button has been clicked!");
        }
    });
}
