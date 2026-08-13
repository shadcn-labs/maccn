//! A macOS segmented control.

use gpui::{
    App, AnyElement, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_base::Button as BaseButton;

use crate::{
    CONTROL_TEXT_WEIGHT, MacControlSize, control_height, control_radius, control_text_size,
    theme::ThemeExt as _,
};

/// A single segment in a [`MacSegmentedControl`].
#[derive(IntoElement)]
pub struct MacSegment {
    inner: BaseButton,
    selected: bool,
    size: MacControlSize,
}

impl MacSegment {
    /// Creates a segment with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: BaseButton::new(id.clone()),
            selected: false,
            size: MacControlSize::Regular,
        }
    }

    /// Marks the segment as selected.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the AppKit control size.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Disables the segment.
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            inner: self.inner.disabled(disabled),
            ..self
        }
    }

    /// Sets the activation handler.
    pub fn on_click(
        self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            inner: self.inner.on_click(handler),
            ..self
        }
    }

    /// Sets the accessible name.
    pub fn accessibility_label(self, label: impl Into<SharedString>) -> Self {
        Self {
            inner: self.inner.accessibility_label(label),
            ..self
        }
    }
}

impl Styled for MacSegment {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacSegment {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacSegment {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacSegment {}

impl RenderOnce for MacSegment {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let selected = self.selected;
        let size = self.size;
        let padding_x = match size {
            MacControlSize::ExtraLarge => 10.,
            MacControlSize::Large => 8.,
            MacControlSize::Regular => 6.,
            MacControlSize::Small => 5.5,
            MacControlSize::Mini => 4.5,
        };

        self.inner
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .h_full()
            .px(px(padding_x))
            .text_size(px(control_text_size(size)))
            .font_weight(CONTROL_TEXT_WEIGHT)
            .text_color(if selected {
                theme.label_on_accent
            } else {
                theme.label
            })
            .when(selected, |this| this.bg(theme.accent))
            .active(|style| {
                style.bg(if selected {
                    theme.segmented_on_pressed
                } else {
                    theme.control_pressed
                })
            })
            .focus_visible(|style| style.shadow(crate::focus_ring_shadow(theme.focus_ring)))
    }
}

/// A macOS segmented control container.
#[derive(IntoElement)]
pub struct MacSegmentedControl {
    inner: gpui::Stateful<gpui::Div>,
    size: MacControlSize,
    disabled: bool,
}

impl MacSegmentedControl {
    /// Creates a segmented control with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: div().id(id.clone()),
            size: MacControlSize::Regular,
            disabled: false,
        }
    }

    /// Sets the AppKit control size.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Disables every segment.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for MacSegmentedControl {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacSegmentedControl {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacSegmentedControl {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacSegmentedControl {}

impl RenderOnce for MacSegmentedControl {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let size = self.size;
        self.inner
            .flex()
            .flex_none()
            .items_stretch()
            .h(px(control_height(size)))
            .rounded(px(control_radius(size)))
            .overflow_hidden()
            .bg(theme.segmented_container)
            .when(self.disabled, |this| this.opacity(0.5))
    }
}
