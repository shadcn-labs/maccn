//! A macOS text label.

use gpui::{
    App, AnyElement, ElementId, FontWeight, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    div, px,
};

use crate::theme::ThemeExt as _;

/// The macOS text styles carried by `NSFont.textStyle` labels.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LabelStyle {
    #[default]
    Body,
    LargeTitle,
    Title1,
    Title2,
    Title3,
    Headline,
    Callout,
    Subheadline,
    Footnote,
    Caption1,
    Caption2,
}

impl LabelStyle {
    fn metrics(self) -> (f32, FontWeight) {
        match self {
            Self::LargeTitle => (26., FontWeight::NORMAL),
            Self::Title1 => (22., FontWeight::NORMAL),
            Self::Title2 => (17., FontWeight::NORMAL),
            Self::Title3 => (15., FontWeight::NORMAL),
            Self::Headline => (13., FontWeight::BOLD),
            Self::Body => (13., FontWeight::NORMAL),
            Self::Callout => (12., FontWeight::NORMAL),
            Self::Subheadline => (11., FontWeight::NORMAL),
            Self::Footnote => (10., FontWeight::NORMAL),
            Self::Caption1 => (10., FontWeight::NORMAL),
            Self::Caption2 => (10., FontWeight::NORMAL),
        }
    }
}

/// A macOS text label.
#[derive(IntoElement)]
pub struct MacLabel {
    inner: gpui::Stateful<gpui::Div>,
    style: LabelStyle,
    secondary: bool,
}

impl MacLabel {
    /// Creates a body label with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: div().id(id.clone()),
            style: LabelStyle::Body,
            secondary: false,
        }
    }

    /// Sets the text style.
    pub fn style(mut self, style: LabelStyle) -> Self {
        self.style = style;
        self
    }

    /// Uses the secondary label color.
    pub fn secondary(mut self, secondary: bool) -> Self {
        self.secondary = secondary;
        self
    }
}

impl Styled for MacLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacLabel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacLabel {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacLabel {}

impl RenderOnce for MacLabel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let (size, weight) = self.style.metrics();
        let color = if self.secondary {
            theme.label_secondary
        } else {
            theme.label
        };
        self.inner.text_size(px(size)).font_weight(weight).text_color(color)
    }
}
