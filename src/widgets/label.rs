//! A label widget.

use crate::render::prelude::*;
use druid::text::TextStorage;
use druid::{
    ArcStr, Color, Data, FontDescriptor, KeyOrValue, LocalizedString, Point, TextAlignment,
    TextLayout,
};
use std::ops::{Deref, DerefMut};

// added padding between the edges of the widget and the text.
const LABEL_X_PADDING: f64 = 2.0;

/// A widget that displays text data.
///
/// This requires the `Data` to be `ArcStr`; to handle static, dynamic, or
/// localized text, use [`Label`].
///
/// [`Label`]: struct.Label.html
#[derive(Default, Debug, Clone)]
pub struct Label {
    layout: TextLayout<String>,
    line_break_mode: LineBreaking,
}

impl Properties for Label {
    type Object = Label;
}

/// Options for handling lines that are too wide for the label.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineBreaking {
    /// Lines are broken at word boundaries.
    WordWrap,
    /// Lines are truncated to the width of the label.
    Clip,
    /// Lines overflow the label.
    Overflow,
}

impl Default for LineBreaking {
    fn default() -> Self {
        LineBreaking::Clip
    }
}

impl Label {
    /// Create a new `Label`.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            layout: TextLayout::from_text(text),
            line_break_mode: LineBreaking::Overflow,
        }
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.layout.set_text_color(color);
        self
    }

    pub fn text_size(mut self, size: f64) -> Self {
        self.layout.set_text_size(size);
        self
    }

    pub fn font(mut self, font: FontDescriptor) -> Self {
        self.layout.set_font(font);
        self
    }

    pub fn line_break_mode(mut self, mode: LineBreaking) -> Self {
        self.line_break_mode = mode;
        self
    }

    pub fn text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.layout.set_text_alignment(alignment);
        self
    }

    #[track_caller]
    pub fn build(self, cx: &mut Cx) {
        let caller = Location::caller().into();
        cx.render_object(caller, self, |_| {});
    }
}

impl Label {
    /// Draw this label's text at the provided `Point`, without internal padding.
    ///
    /// This is a convenience for widgets that want to use Label as a way
    /// of managing a dynamic or localized string, but want finer control
    /// over where the text is drawn.
    fn draw_at(&self, ctx: &mut PaintCtx, origin: impl Into<Point>) {
        debug_assert!(
            self.layout.layout().is_some(),
            "TextLayout::draw called without rebuilding layout object. Text was '{}'",
            self.layout
                .text()
                .as_ref()
                .map(|t| t.as_str())
                .unwrap_or("layout is missing text")
        );
        if let Some(layout) = self.layout.layout() {
            ctx.draw_text(layout, origin);
        }
    }

    /// Return the offset of the first baseline relative to the bottom of the widget.
    fn baseline_offset(&self) -> f64 {
        let text_metrics = self.layout.layout_metrics();
        text_metrics.size.height - text_metrics.first_baseline
    }
}

impl RenderObject for Label {
    type Props = Label;
    type Action = ();

    fn update(&mut self, ctx: &mut UpdateCtx, props: Label) {
        if self.layout.text() != props.layout.text() {
            ctx.request_layout();
            self.layout = props.layout;
        }
        if self.layout.layout().is_none() {
            ctx.request_layout();
        }
    }

    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, children: &mut Children) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle) {}

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        children: &mut Children,
    ) -> Size {
        bc.debug_check("Label");

        let width = match self.line_break_mode {
            LineBreaking::WordWrap => bc.max().width - LABEL_X_PADDING * 2.0,
            _ => f64::INFINITY,
        };

        self.layout.set_wrap_width(width);
        let env = ctx.env().clone();
        self.layout.rebuild_if_needed(ctx.text(), &env);

        let text_metrics = self.layout.layout_metrics();
        ctx.set_baseline_offset(text_metrics.size.height - text_metrics.first_baseline);
        bc.constrain(Size::new(
            text_metrics.size.width + 2. * LABEL_X_PADDING,
            text_metrics.size.height,
        ))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, children: &mut Children) {
        let origin = Point::new(LABEL_X_PADDING, 0.0);
        let label_size = ctx.size();

        if self.line_break_mode == LineBreaking::Clip {
            ctx.clip(label_size.to_rect());
        }
        self.draw_at(ctx, origin)
    }
}
