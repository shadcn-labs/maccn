//! A macOS help button.

use gpui::{
    App, AnyElement, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    px,
};
use gpui_base::Button as BaseButton;

use crate::{
    CONTROL_TEXT_WEIGHT, MacControlSize, control_height, focus_ring_shadow,
    theme::ThemeExt as _,
};

/// A circular question-mark button.
#[derive(IntoElement)]
pub struct MacHelpButton {
    inner: BaseButton,
    size: MacControlSize,
}

impl MacHelpButton {
    /// Creates a help button with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: BaseButton::new(id.clone()),
            size: MacControlSize::Regular,
        }
    }

    /// Sets the AppKit control size.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Disables the button.
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

impl Styled for MacHelpButton {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacHelpButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacHelpButton {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacHelpButton {}

impl RenderOnce for MacHelpButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let size = self.size;
        let diameter = control_height(size);
        let glyph_size = match size {
            MacControlSize::ExtraLarge => 19.,
            MacControlSize::Large => 15.,
            MacControlSize::Regular => 13.,
            MacControlSize::Small => 11.,
            MacControlSize::Mini => 9.,
        };

        self.inner
            .styles(|styles| {
                styles.disabled(|style| {
                    style.bg(theme.control_disabled).text_color(theme.label_disabled)
                })
            })
            .flex()
            .items_center()
            .justify_center()
            .size(px(diameter))
            .rounded_full()
            .bg(theme.control)
            .text_color(theme.label)
            .text_size(px(glyph_size))
            .font_weight(CONTROL_TEXT_WEIGHT)
            .active(|style| style.bg(theme.control_pressed))
            .focus_visible(|style| style.shadow(focus_ring_shadow(theme.focus_ring)))
            .child("?")
    }
}
