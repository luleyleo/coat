use coat::{
    app::App,
    cx::Cx,
    states::mutable::Mutable,
    widgets::{button::Button, padding::Padding},
};

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

fn app(cx: &mut Cx) {
    Padding::new(100.0).build(cx, |cx| {
        Mutable::new().use_in(cx, |cx, count: &mut usize| {
            if Button::new().labeled(cx, format!("Clicked {} times!", count)) {
                println!("The Button has been clicked!");
                *count += 1;
            }
        });
    });
}
