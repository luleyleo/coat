use coat::{
    app::App,
    state::Mutable,
    ui::Ui,
    widgets::{
        flex::{CrossAxisAlignment, Flex, MainAxisAlignment},
        Button, Label, Padding, SizedBox, TextBox,
    },
};

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

fn app(ui: &mut Ui) {
    Padding::new(10.0).build(ui, |ui| {
        Flex::column()
            .main_axis_alignment(MainAxisAlignment::Start)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .build(ui, |ui| {
                Mutable::with(|| String::from("Hello world!")).use_in(
                    ui,
                    |ui, text: &mut String| {
                        Label::new(&*text).build(ui);
                        SizedBox::new().height(5.0).empty(ui);
                        TextBox::new(text).build(ui);
                    },
                );

                SizedBox::new().height(10.0).empty(ui);

                fn btn(name: &str, count: usize) -> String {
                    format!("{} button clicked {} times", name, count)
                }

                Mutable::new().use_in(ui, |ui, linked_count: &mut usize| {
                    Label::new("Some buttons placed manually:").build(ui);
                    SizedBox::new().height(10.0).empty(ui);
                    if Button::new().labeled(ui, btn("Linked", *linked_count)) {
                        *linked_count += 1;
                    }
                    SizedBox::new().height(20.0).empty(ui);
                    Mutable::new().use_in(ui, |ui, count: &mut usize| {
                        if Button::new().labeled(ui, btn("Lonely", *count)) {
                            *count += 1;
                        }
                        if *count % 2 == 1 {
                            SizedBox::new().height(10.0).empty(ui);
                            Mutable::new().use_in(ui, |ui, count: &mut usize| {
                                if Button::new().labeled(ui, btn("Conditional", *count)) {
                                    *count += 1;
                                }
                            });
                        }
                    });
                    SizedBox::new().height(20.0).empty(ui);
                    if Button::new().labeled(ui, btn("Linked", *linked_count)) {
                        *linked_count += 1;
                    }
                });

                SizedBox::new().height(10.0).empty(ui);

                Mutable::with(|| vec![1, 2, 3]).use_in(ui, |ui, buttons| {
                    Flex::row()
                        .cross_axis_alignment(CrossAxisAlignment::Center)
                        .build(ui, |ui| {
                            Label::new("More buttons in a loop:").build(ui);
                            SizedBox::new().width(5.0).empty(ui);
                            if Button::new().style(styles::AddButton).labeled(ui, "Add") {
                                buttons.push(3);
                            }
                        });

                    for i in buttons.iter_mut() {
                        SizedBox::new().height(10.0).empty(ui);
                        if Button::new().labeled(ui, format!("{} hits left", i)) {
                            *i -= 1;
                        }
                    }

                    buttons.retain(|&life| life > 0);
                });
            });
    });
}

mod styles {
    use coat::{
        kurbo::{Size, Vec2},
        piet::Color,
        widgets::button,
        BoxConstraints,
    };

    pub struct AddButton;
    impl button::StyleSheet for AddButton {
        fn eq(&self, other: &dyn button::StyleSheet) -> bool {
            std::any::Any::type_id(other) == std::any::TypeId::of::<Self>()
        }

        fn enabled(&self) -> button::Style {
            button::Style {
                shadow_offset: Vec2::new(0.0, 0.0),
                background: Color::rgba(0.0, 0.0, 0.0, 0.0),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: Color::rgb(0.5, 0.5, 0.87),
                text_color: Color::WHITE,
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: Color::rgb(0.6, 0.6, 0.87),
                ..self.enabled()
            }
        }

        fn pick_size(&self, _bc: &BoxConstraints, required_size: Size) -> Size {
            Size::new(required_size.width, required_size.height)
        }
    }
}
