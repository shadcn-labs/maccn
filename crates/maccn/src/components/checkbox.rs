//! A macOS checkbox.

use gpui::{
    App, AnyElement, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_base::{
    Checkbox as BaseCheckbox, CheckboxIndicator, CheckboxState,
};

use crate::{
    MacControlSize, check_mark, control_text_size, dash_mark, focus_ring_shadow,
    theme::ThemeExt as _, CONTROL_TEXT_WEIGHT,
};

/// Re-exported checkbox value.
pub type MacCheckboxState = CheckboxState;

fn geometry(size: MacControlSize) -> (f32, f32, f32, f32, f32, f32, f32) {
    match size {
        MacControlSize::ExtraLarge => (18., 6.5, 11.7, 11.3, 8., 2., 7.),
        MacControlSize::Large => (18., 6.5, 11.7, 11.3, 8., 2., 5.),
        MacControlSize::Regular => (16., 5.5, 9.3, 8.9, 6.5, 2., 5.),
        MacControlSize::Small => (14., 4.5, 9.3, 8.9, 6.5, 2., 4.),
        MacControlSize::Mini => (12., 3.5, 7.9, 7.6, 5.5, 1.7, 3.),
    }
}

/// A macOS checkbox with AppKit metrics.
#[derive(IntoElement)]
pub struct MacCheckbox {
    inner: BaseCheckbox,
    size: MacControlSize,
    state: CheckboxState,
    children: Vec<AnyElement>,
}

impl MacCheckbox {
    /// Creates an unchecked checkbox with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: BaseCheckbox::new(id.clone()),
            size: MacControlSize::Regular,
            state: CheckboxState::Unchecked,
            children: Vec::new(),
        }
    }

    /// Sets the AppKit control size.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the checkbox value.
    pub fn state(self, state: CheckboxState) -> Self {
        Self {
            inner: self.inner.state(state),
            state,
            ..self
        }
    }

    /// Sets the checkbox to checked.
    pub fn checked(self, checked: bool) -> Self {
        self.state(if checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        })
    }

    /// Sets the checkbox to indeterminate.
    pub fn indeterminate(self, indeterminate: bool) -> Self {
        let state = if indeterminate {
            CheckboxState::Indeterminate
        } else {
            self.state
        };
        self.state(state)
    }

    /// Disables the checkbox.
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            inner: self.inner.disabled(disabled),
            ..self
        }
    }

    /// Handles state changes.
    pub fn on_change(
        self,
        handler: impl Fn(CheckboxState, &ClickEvent, &mut Window, &mut App) + 'static,
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

impl Styled for MacCheckbox {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacCheckbox {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl InteractiveElement for MacCheckbox {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacCheckbox {}

impl RenderOnce for MacCheckbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let size = self.size;
        let state = self.state;
        let (box_size, radius, check_w, check_h, dash_w, dash_h, gap) = geometry(size);
        let checked = matches!(state, CheckboxState::Checked | CheckboxState::Indeterminate);

        let indicator = CheckboxIndicator::new()
            .state(state)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(box_size))
            .rounded(px(radius))
            .bg(theme.control)
            .styles(|styles| {
                styles
                    .checked(|style| style.bg(theme.accent))
                    .disabled(|style| {
                        style
                            .bg(theme.control_disabled_strong)
                            .when(checked, |style| style.bg(theme.accent_disabled_strong))
                    })
            })
            .when(checked, |this| {
                this.child(if state == CheckboxState::Checked {
                    check_mark(theme.label_on_accent, check_w, check_h).into_any_element()
                } else {
                    dash_mark(theme.label_on_accent, dash_w, dash_h).into_any_element()
                })
            });

        self.inner
            .flex()
            .items_center()
            .gap(px(gap))
            .text_size(px(control_text_size(size)))
            .font_weight(CONTROL_TEXT_WEIGHT)
            .text_color(theme.label)
            .focus_visible(|style| style.shadow(focus_ring_shadow(theme.focus_ring)))
            .child(indicator)
            .child(div().flex_none().children(self.children))
    }
}
