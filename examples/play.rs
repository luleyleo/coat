use coat::{
    app::App,
    cx::Cx,
    state::Mutable,
    widgets::{
        flex::{CrossAxisAlignment, Flex, MainAxisAlignment},
        Button, Label, Padding, SizedBox,
    },
};

fn main() {
    App::new("Play").run(app).expect("Failed to launch the app");
}

fn app(cx: &mut Cx) {
    Padding::new(10.0).build(cx, |cx| {
        Flex::column()
            .main_axis_alignment(MainAxisAlignment::Start)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .build(cx, |cx| {
                fn btn(name: &str, count: usize) -> String {
                    format!("{} button clicked {} times", name, count)
                }

                Mutable::new().use_in(cx, |cx, linked_count: &mut usize| {
                    Label::new("Some buttons placed manually:").build(cx);
                    SizedBox::new().height(10.0).empty(cx);
                    if Button::new().labeled(cx, btn("Linked", *linked_count)) {
                        *linked_count += 1;
                    }
                    SizedBox::new().height(20.0).empty(cx);
                    Mutable::new().use_in(cx, |cx, count: &mut usize| {
                        if Button::new().labeled(cx, btn("Lonely", *count)) {
                            *count += 1;
                        }
                        if *count % 2 == 1 {
                            SizedBox::new().height(10.0).empty(cx);
                            Mutable::new().use_in(cx, |cx, count: &mut usize| {
                                if Button::new().labeled(cx, btn("Conditional", *count)) {
                                    *count += 1;
                                }
                            });
                        }
                    });
                    SizedBox::new().height(20.0).empty(cx);
                    if Button::new().labeled(cx, btn("Linked", *linked_count)) {
                        *linked_count += 1;
                    }
                });

                SizedBox::new().height(10.0).empty(cx);

                Mutable::with(|| vec![1, 2, 3]).use_in(cx, |cx, buttons| {
                    Flex::row()
                        .cross_axis_alignment(CrossAxisAlignment::Center)
                        .build(cx, |cx| {
                            Label::new("More buttons in a loop:").build(cx);
                            SizedBox::new().width(5.0).empty(cx);
                            if Button::new().style(styles::AddButton).labeled(cx, "Add") {
                                buttons.push(3);
                            }
                        });

                    for i in buttons.iter_mut() {
                        SizedBox::new().height(10.0).empty(cx);
                        if Button::new().labeled(cx, format!("{} hits left", i)) {
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
