// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

    #[track_caller]
    pub fn build(self, cx: &mut Cx) {
        let caller = Location::caller().into();
        cx.render_object(caller, self, |_| {});
    }

    /// Builder-style method for setting the text color.
    ///
    /// The argument can be either a `Color` or a [`Key<Color>`].
    ///
    /// [`Key<Color>`]: ../struct.Key.html
    pub fn with_text_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.set_text_color(color);
        self
    }

    /// Builder-style method for setting the text size.
    ///
    /// The argument can be either an `f64` or a [`Key<f64>`].
    ///
    /// [`Key<f64>`]: ../struct.Key.html
    pub fn with_text_size(mut self, size: impl Into<KeyOrValue<f64>>) -> Self {
        self.set_text_size(size);
        self
    }

    /// Builder-style method for setting the font.
    ///
    /// The argument can be a [`FontDescriptor`] or a [`Key<FontDescriptor>`]
    /// that refers to a font defined in the [`Env`].
    ///
    /// [`Env`]: ../struct.Env.html
    /// [`FontDescriptor`]: ../struct.FontDescriptor.html
    /// [`Key<FontDescriptor>`]: ../struct.Key.html
    pub fn with_font(mut self, font: impl Into<KeyOrValue<FontDescriptor>>) -> Self {
        self.set_font(font);
        self
    }

    /// Builder-style method to set the [`LineBreaking`] behaviour.
    ///
    /// [`LineBreaking`]: enum.LineBreaking.html
    pub fn with_line_break_mode(mut self, mode: LineBreaking) -> Self {
        self.set_line_break_mode(mode);
        self
    }

    /// Builder-style method to set the [`TextAlignment`].
    ///
    /// [`TextAlignment`]: enum.TextAlignment.html
    pub fn with_text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.set_text_alignment(alignment);
        self
    }

    /// Set the text color.
    ///
    /// The argument can be either a `Color` or a [`Key<Color>`].
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`Key<Color>`]: ../struct.Key.html
    pub fn set_text_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        self.layout.set_text_color(color);
    }

    /// Set the text size.
    ///
    /// The argument can be either an `f64` or a [`Key<f64>`].
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`Key<f64>`]: ../struct.Key.html
    pub fn set_text_size(&mut self, size: impl Into<KeyOrValue<f64>>) {
        self.layout.set_text_size(size);
    }

    /// Set the font.
    ///
    /// The argument can be a [`FontDescriptor`] or a [`Key<FontDescriptor>`]
    /// that refers to a font defined in the [`Env`].
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`Env`]: ../struct.Env.html
    /// [`FontDescriptor`]: ../struct.FontDescriptor.html
    /// [`Key<FontDescriptor>`]: ../struct.Key.html
    pub fn set_font(&mut self, font: impl Into<KeyOrValue<FontDescriptor>>) {
        self.layout.set_font(font);
    }

    /// Set the [`LineBreaking`] behaviour.
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`LineBreaking`]: enum.LineBreaking.html
    pub fn set_line_break_mode(&mut self, mode: LineBreaking) {
        self.line_break_mode = mode;
    }

    /// Set the [`TextAlignment`] for this layout.
    ///
    /// [`TextAlignment`]: enum.TextAlignment.html
    pub fn set_text_alignment(&mut self, alignment: TextAlignment) {
        self.layout.set_text_alignment(alignment);
    }

    /// Draw this label's text at the provided `Point`, without internal padding.
    ///
    /// This is a convenience for widgets that want to use Label as a way
    /// of managing a dynamic or localized string, but want finer control
    /// over where the text is drawn.
    pub fn draw_at(&self, ctx: &mut PaintCtx, origin: impl Into<Point>) {
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
    pub fn baseline_offset(&self) -> f64 {
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
        self.layout
            .rebuild_if_needed(ctx.text(), &druid::Env::default());

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
