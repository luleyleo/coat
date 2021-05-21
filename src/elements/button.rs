use crate::{
    constraints::Constraints,
    event::Event,
    kurbo::{RoundedRect, Size},
    piet::{
        Color, Piet, PietText, PietTextLayout, RenderContext, Text, TextAlignment, TextLayout,
        TextLayoutBuilder,
    },
    tree::{Content, Element},
    ui::Ui,
};

#[derive(Default)]
pub struct ButtonElement {
    pub text: String,
    pub layout: Option<PietTextLayout>,
    pub pressed: bool,
    pub clicks: u32,
}

impl Element for ButtonElement {
    fn paint(&mut self, piet: &mut Piet, size: Size, _content: &mut Content) {
        let layout = self.layout.as_ref().unwrap();
        let color = if self.pressed {
            Color::rgb(0.6, 0.6, 0.95)
        } else {
            Color::rgb(0.5, 0.5, 0.87)
        };
        piet.fill(&RoundedRect::from_rect(size.to_rect(), 5.0), &color);
        let offset = (
            (size.width - layout.size().width) / 2.0,
            (size.height - layout.size().height) / 2.0,
        );
        piet.draw_text(layout, offset)
    }

    fn layout(&mut self, constraints: &Constraints, _: &mut Content, text: &mut PietText) -> Size {
        if self.layout.is_none() {
            self.layout = Some(
                text.new_text_layout(self.text.clone())
                    .max_width(constraints.max.width)
                    .text_color(Color::WHITE)
                    .alignment(TextAlignment::Center)
                    .build()
                    .unwrap(),
            );
        }
        constraints.max
    }

    fn event(&mut self, event: &Event, handled: &mut bool, _content: &mut Content) {
        match event {
            Event::MouseDown(_) => {
                *handled |= true;
                self.clicks += 1;
                self.pressed = true;
                println!("CLICKED");
            }
            Event::MouseUp(_) => {
                self.pressed = false;
            }
            _ => {}
        }
    }
}

#[track_caller]
pub fn button(ui: &mut Ui, text: &str) {
    let location = std::panic::Location::caller();
    ui.add(
        location,
        |button: &mut ButtonElement| {
            if button.text != text {
                button.text = text.to_string();
                button.layout = None;
            }
        },
        |_| {},
    );
}
