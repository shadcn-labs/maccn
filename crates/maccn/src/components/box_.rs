//! A macOS group box.

use gpui::{
    App, AnyElement, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::theme::ThemeExt as _;

/// A grouped content box with an optional footnote caption.
#[derive(IntoElement)]
pub struct MacBox {
    inner: gpui::Stateful<gpui::Div>,
}

impl MacBox {
    /// Creates a box with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: div().id(id.clone()),
        }
    }
}

impl Styled for MacBox {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacBox {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacBox {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacBox {}

impl RenderOnce for MacBox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        self.inner
            .flex()
            .flex_col()
            .p(px(12.))
            .rounded(px(12.))
            .bg(theme.group_box)
    }
}
