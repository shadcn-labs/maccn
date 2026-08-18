//! macOS radio buttons and radio groups.

use gpui::{
    App, AnyElement, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    Axis, div, prelude::FluentBuilder as _, px,
};
use gpui_base::{Radio as BaseRadio, RadioGroup as BaseRadioGroup};

use crate::{MacControlSize, control_text_size, focus_ring_shadow, theme::ThemeExt as _, CONTROL_TEXT_WEIGHT};

fn geometry(size: MacControlSize) -> (f32, f32, f32) {
    match size {
        MacControlSize::ExtraLarge => (18., 5., 7.),
        MacControlSize::Large => (18., 5., 5.),
        MacControlSize::Regular => (16., 4.8, 3.),
        MacControlSize::Small => (14., 4.8, 3.),
        MacControlSize::Mini => (12., 4., 3.),
    }
}

/// A macOS radio button.
#[derive(IntoElement)]
pub struct MacRadio {
    inner: BaseRadio,
    id: ElementId,
    size: MacControlSize,
    checked: bool,
    disabled: bool,
    children: Vec<AnyElement>,
}

impl MacRadio {
    /// Creates a radio with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: BaseRadio::new(id.clone()),
            id,
            size: MacControlSize::Regular,
            checked: false,
            disabled: false,
            children: Vec::new(),
        }
    }

    /// Sets the AppKit control size.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Sets whether the radio is checked.
    pub fn checked(self, checked: bool) -> Self {
        Self {
            inner: self.inner.checked(checked),
            checked,
            ..self
        }
    }

    /// Disables the radio.
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            inner: self.inner.disabled(disabled),
            disabled,
            ..self
        }
    }

    /// Handles selection changes.
    pub fn on_change(
        self,
        handler: impl Fn(bool, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            inner: self.inner.on_change(handler),
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

impl Styled for MacRadio {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacRadio {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl InteractiveElement for MacRadio {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacRadio {}

impl RenderOnce for MacRadio {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let checked = self.checked;
        let disabled = self.disabled;
        let size = self.size;
        let (radio_size, dot_size, gap) = geometry(size);

        let control = div()
            .id((self.id.clone(), "control"))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .size(px(radio_size))
            .rounded_full()
            .bg(if disabled {
                if checked {
                    theme.accent_disabled_strong
                } else {
                    theme.control_disabled_strong
                }
            } else {
                theme.control
            })
            .when(checked && !disabled, |this| this.bg(theme.accent))
            .active(|style| {
                if disabled {
                    style
                } else {
                    style.bg(if checked {
                        theme.accent_pressed
                    } else {
                        theme.control_pressed
                    })
                }
            })
            .when(checked, |this| {
                this.child(
                    div()
                        .flex_none()
                        .size(px(dot_size))
                        .rounded_full()
                        .bg(theme.label_on_accent),
                )
            });

        self.inner
            .flex()
            .items_center()
            .gap(px(gap))
            .text_size(px(control_text_size(size)))
            .font_weight(CONTROL_TEXT_WEIGHT)
            .text_color(theme.label)
            .focus_visible(|style| style.shadow(focus_ring_shadow(theme.focus_ring)))
            .child(control)
            .child(div().flex_none().children(self.children))
    }
}

/// A macOS radio group.
#[derive(IntoElement)]
pub struct MacRadioGroup {
    inner: BaseRadioGroup,
    size: MacControlSize,
    gap: Option<f32>,
}

impl MacRadioGroup {
    /// Creates a vertical radio group.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: BaseRadioGroup::new(id.clone()),
            size: MacControlSize::Regular,
            gap: None,
        }
    }

    /// Sets the AppKit control size of the contained radios.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the layout axis.
    pub fn axis(mut self, axis: Axis) -> Self {
        self.inner = self.inner.axis(axis);
        self
    }

    /// Overrides the spacing between rows.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap);
        self
    }
}

impl Styled for MacRadioGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacRadioGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacRadioGroup {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacRadioGroup {}

impl RenderOnce for MacRadioGroup {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let gap = self.gap.unwrap_or(8.);
        self.inner.flex().flex_col().gap(px(gap))
    }
}
