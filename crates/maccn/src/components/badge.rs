//! A macOS badge.

use gpui::{
    App, AnyElement, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::{CONTROL_TEXT_WEIGHT, theme::ThemeExt as _};

/// A small rounded label used to annotate controls.
#[derive(IntoElement)]
pub struct MacBadge {
    inner: gpui::Stateful<gpui::Div>,
}

impl MacBadge {
    /// Creates a badge with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: div().id(id.clone()),
        }
    }
}

impl Styled for MacBadge {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacBadge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacBadge {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacBadge {}

impl RenderOnce for MacBadge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        self.inner
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .h(px(16.))
            .px(px(5.))
            .rounded_full()
            .bg(theme.control)
            .text_color(theme.label_secondary)
            .text_size(px(11.))
            .font_weight(CONTROL_TEXT_WEIGHT)
    }
}
