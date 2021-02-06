use coat::{
    app::App,
    cx::Cx,
    states::mutable::Mutable,
    widgets::{
        button::Button,
        flex::{CrossAxisAlignment, Flex, MainAxisAlignment},
        SizedBox,
    },
};

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

fn app(cx: &mut Cx) {
    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .build(cx, |cx| {
            Mutable::new().use_in(cx, |cx, two_count: &mut usize| {
                if Button::new().labeled(cx, format!("Clicked {} times!", two_count)) {
                    *two_count += 1;
                }
                SizedBox::new().height(10.0).empty(cx);
                Mutable::new().use_in(cx, |cx, count: &mut usize| {
                    if Button::new().labeled(cx, format!("Clicked {} times!", count)) {
                        *count += 1;
                    }
                });
                SizedBox::new().height(10.0).empty(cx);
                if Button::new().labeled(cx, format!("Clicked {} times!", two_count)) {
                    *two_count += 1;
                }
                SizedBox::new().height(10.0).empty(cx);
                Mutable::new().use_in(cx, |cx, count: &mut usize| {
                    if Button::new().labeled(cx, format!("Clicked {} times!", count)) {
                        *count += 1;
                    }
                });
            });
        });
}
