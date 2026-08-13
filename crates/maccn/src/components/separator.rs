//! A macOS separator.

use gpui::{
    App, AnyElement, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, Axis, div, px,
};

use crate::theme::ThemeExt as _;

/// A one-pixel horizontal or vertical separator.
#[derive(IntoElement)]
pub struct MacSeparator {
    inner: gpui::Stateful<gpui::Div>,
    axis: Axis,
}

impl MacSeparator {
    /// Creates a horizontal separator with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: div().id(id.clone()),
            axis: Axis::Horizontal,
        }
    }

    /// Sets the layout axis.
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }
}

impl Styled for MacSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacSeparator {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacSeparator {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacSeparator {}

impl RenderOnce for MacSeparator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let color = theme.separator;
        match self.axis {
            Axis::Horizontal => self.inner.w_full().h(px(1.)).bg(color),
            Axis::Vertical => self.inner.h_full().w(px(1.)).bg(color),
        }
    }
}
