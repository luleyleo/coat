use crate::{
    constraints::Constraints,
    context::ElementCtx,
    event::Event,
    kurbo::{RoundedRect, Size},
    piet::{
        Color, Piet, PietText, PietTextLayout, RenderContext, Text, TextAlignment, TextLayout,
        TextLayoutBuilder,
    },
    tree::{Content, Element, Handled},
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
    fn paint(&mut self, element: &mut ElementCtx, piet: &mut Piet, _content: &mut Content) {
        let layout = self.layout.as_ref().unwrap();
        let size = element.size();

        let color = if self.pressed {
            Color::rgb(0.6, 0.6, 0.95)
        } else {
            Color::rgb(0.5, 0.5, 0.87)
        };

        let text_offset = (
            (size.width - layout.size().width) / 2.0,
            (size.height - layout.size().height) / 2.0,
        );

        piet.fill(&RoundedRect::from_rect(size.to_rect(), 5.0), &color);
        piet.draw_text(layout, text_offset)
    }

    fn layout(
        &mut self,
        _element: &mut ElementCtx,
        constraints: &Constraints,
        _content: &mut Content,
        text: &mut PietText,
    ) -> Size {
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

    fn event(
        &mut self,
        element: &mut ElementCtx,
        event: &Event,
        _content: &mut Content,
    ) -> Handled {
        match event {
            Event::MouseDown(_) => {
                self.clicks += 1;
                self.pressed = true;
                element.request_im_pass();
                Handled(true)
            }
            Event::MouseUp(_) => {
                self.pressed = false;
                Handled(true)
            }
            _ => Handled(false),
        }
    }
}

#[track_caller]
pub fn button(ui: &mut Ui, text: &str) -> bool {
    let location = std::panic::Location::caller();
    let mut clicks = 0;

    ui.add(
        location,
        |button: &mut ButtonElement| {
            if button.text != text {
                button.text = text.to_string();
                button.layout = None;
            }

            clicks = button.clicks;
            button.clicks = 0;
        },
        |_| {},
    );

    if clicks > 0 {
        ui.action_emitted();
    }

    clicks > 0
}
