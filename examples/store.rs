use coat::{
    app::App,
    state,
    ui::Ui,
    widgets::{
        flex::{CrossAxisAlignment, Flex, MainAxisAlignment},
        Button, Label, SizedBox,
    },
};

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

enum CountMsg {
    Set(i32),
    Increment,
    Decrement,
}

fn update_count(count: &mut i32, msg: CountMsg) {
    match msg {
        CountMsg::Set(value) => *count = value,
        CountMsg::Increment => *count += 1,
        CountMsg::Decrement => *count -= 1,
    }
}

fn app(ui: &mut Ui) {
    state::use_store(
        ui,
        || 0,
        update_count,
        |ui, count| {
            Flex::column()
                .main_axis_alignment(MainAxisAlignment::Center)
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .build(ui, |ui| {
                    Label::new(format!("Current count: {}", count.state)).build(ui);
                    SizedBox::new().height(10.0).empty(ui);
                    if Button::new().labeled(ui, "Increment") {
                        count.msg.push(CountMsg::Increment);
                    }
                    SizedBox::new().height(5.0).empty(ui);
                    if Button::new().labeled(ui, "Decrement") {
                        count.msg.push(CountMsg::Decrement);
                    }
                    SizedBox::new().height(10.0).empty(ui);
                    if Button::new().labeled(ui, "Reset") {
                        count.msg.push(CountMsg::Set(0));
                    }
                });
        },
    );
}
