pub mod button {
    use shell::piet::RenderContext;

    use crate::{
        constraints::Constraints,
        context::ElementCtx,
        kurbo::Size,
        piet::{Color, Piet, PietText},
        tree::{Content, Element, Handled},
    };

    pub struct ButtonElement {
        pub color: Color,
    }

    impl Default for ButtonElement {
        fn default() -> Self {
            ButtonElement {
                color: Color::WHITE,
            }
        }
    }

    impl ButtonElement {
        pub fn new(color: Color) -> Self {
            ButtonElement { color }
        }
    }

    impl Element for ButtonElement {
        fn paint(&mut self, element: &mut ElementCtx, piet: &mut Piet, _content: &mut Content) {
            piet.fill(&element.size.to_rect(), &self.color);
        }

        fn layout(
            &mut self,
            _element: &mut ElementCtx,
            constraints: &Constraints,
            _content: &mut Content,
            _text: &mut PietText,
        ) -> Size {
            constraints.max
        }

        fn event(
            &mut self,
            _element: &mut ElementCtx,
            _event: &crate::event::Event,
            _content: &mut Content,
        ) -> Handled {
            Handled(true)
        }
    }
}
