//! A macOS push button.

use gpui::{
    App, AnyElement, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    px,
};
use gpui_base::Button as BaseButton;

use crate::{
    CONTROL_TEXT_WEIGHT, DISABLED_OPACITY, MacControlSize, control_height, control_radius,
    control_text_size, focus_ring_shadow,
    theme::ThemeExt as _,
};

/// The visual variants of a macOS push button.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonVariant {
    /// A bordered neutral button.
    #[default]
    Bordered,
    /// An accent-filled button.
    Prominent,
    /// A bordered destructive button.
    Destructive,
}

/// A macOS push button with AppKit metrics.
#[derive(IntoElement)]
pub struct MacButton {
    inner: BaseButton,
    variant: ButtonVariant,
    size: MacControlSize,
}

impl MacButton {
    /// Creates a bordered button with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: BaseButton::new(id.clone()),
            variant: ButtonVariant::Bordered,
            size: MacControlSize::Regular,
        }
    }

    /// Sets the visual variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
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

impl Styled for MacButton {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacButton {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacButton {}

impl RenderOnce for MacButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let variant = self.variant;
        let size = self.size;
        let padding_x = match size {
            MacControlSize::ExtraLarge | MacControlSize::Large | MacControlSize::Regular => 16.,
            MacControlSize::Small => 10.,
            MacControlSize::Mini => 7.,
        };

        self.inner
            .styles(|styles| {
                styles.disabled(|style| match variant {
                    ButtonVariant::Bordered => style
                        .bg(theme.control_disabled)
                        .text_color(theme.label_disabled),
                    ButtonVariant::Prominent => style
                        .bg(theme.accent_disabled)
                        .text_color(theme.label_on_accent_disabled),
                    ButtonVariant::Destructive => style
                        .bg(theme.destructive_bg)
                        .text_color(theme.destructive)
                        .opacity(DISABLED_OPACITY),
                })
            })
            .flex()
            .items_center()
            .justify_center()
            .h(px(control_height(size)))
            .px(px(padding_x))
            .rounded(px(control_radius(size)))
            .font_weight(CONTROL_TEXT_WEIGHT)
            .text_size(px(control_text_size(size)))
            .text_color(match variant {
                ButtonVariant::Bordered => theme.label,
                ButtonVariant::Prominent => theme.label_on_accent,
                ButtonVariant::Destructive => theme.destructive,
            })
            .bg(match variant {
                ButtonVariant::Bordered => theme.control_button,
                ButtonVariant::Prominent => theme.accent,
                ButtonVariant::Destructive => theme.destructive_bg,
            })
            .active(|style| {
                style.bg(match variant {
                    ButtonVariant::Bordered => theme.control_button_pressed,
                    ButtonVariant::Prominent => theme.accent_pressed_soft,
                    ButtonVariant::Destructive => theme.destructive_bg_pressed,
                })
            })
            .focus_visible(|style| style.shadow(focus_ring_shadow(theme.focus_ring)))
    }
}
