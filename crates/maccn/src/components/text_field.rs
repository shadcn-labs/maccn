//! macOS text fields, search fields, and secure fields.

use gpui::{
    App, AnyElement, ElementId, Entity, Focusable as _, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    div, prelude::FluentBuilder as _, px,
};
use gpui_base::{Input, InputBase, StyledExt as _, input::InputState};

use crate::{
    MacControlSize, close_x, control_height, field_radius, focus_ring_shadow, magnifier,
    theme::ThemeExt as _,
};

/// A macOS text field.
#[derive(IntoElement)]
pub struct MacTextField {
    state: Entity<InputState>,
    id: ElementId,
    size: MacControlSize,
    disabled: bool,
    radius: Option<f32>,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    style: gpui::StyleRefinement,
}

impl MacTextField {
    /// Creates a field driven by the given input state.
    pub fn new(id: impl Into<ElementId>, state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            id: id.into(),
            size: MacControlSize::Regular,
            disabled: false,
            radius: None,
            prefix: None,
            suffix: None,
            style: gpui::StyleRefinement::default(),
        }
    }

    /// Sets the AppKit control size.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Disables the field.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Renders the field as a pill (search style).
    pub fn pill(mut self) -> Self {
        self.radius = Some(crate::PILL);
        self
    }

    /// Adds a leading element.
    pub fn prefix(mut self, element: impl IntoElement) -> Self {
        self.prefix = Some(element.into_any_element());
        self
    }

    /// Adds a trailing element.
    pub fn suffix(mut self, element: impl IntoElement) -> Self {
        self.suffix = Some(element.into_any_element());
        self
    }

    /// Overrides the corner radius.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    fn render_field(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let style = self.style;
        let disabled = self.disabled;
        let size = self.size;
        let height = control_height(size);
        let radius = self.radius.unwrap_or_else(|| field_radius(size));
        let padding_start = match size {
            MacControlSize::ExtraLarge => 10.,
            MacControlSize::Large => 8.,
            MacControlSize::Regular => 8.,
            MacControlSize::Small => 6.,
            MacControlSize::Mini => 6.,
        };
        let padding_end = match size {
            MacControlSize::ExtraLarge => 8.,
            MacControlSize::Large => 6.,
            MacControlSize::Regular => 4.,
            MacControlSize::Small => 2.5,
            MacControlSize::Mini => 2.,
        };

        let base = self.state.read(cx).base_state().clone();
        base.update(cx, |base, cx| base.set_disabled(disabled, cx));

        let focused = self.state.read(cx).focus_handle(cx).is_focused(window) && !disabled;
        let state = self.state.clone();

        InputBase::new((self.id, "frame"))
            .focused(focused)
            .disabled(disabled)
            .w_full()
            .h(px(height))
            .pl(px(padding_start))
            .pr(px(padding_end))
            .flex()
            .items_center()
            .gap(px(4.))
            .rounded(px(radius))
            .bg(theme.control_bg)
            .border_1()
            .border_color(theme.border_control)
            .styles(|styles| {
                styles
                    .focused(|style| {
                        style
                            .border_color(theme.focus_ring_border)
                            .shadow(focus_ring_shadow(theme.focus_ring))
                    })
                    .disabled(|style| style.bg(theme.control_bg_disabled))
            })
            .refine_style(&style)
            .when_some(self.prefix, |this, prefix| this.child(prefix))
            .child(Input::new(&state))
            .when_some(self.suffix, |this, suffix| this.child(suffix))
    }
}

impl RenderOnce for MacTextField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.render_field(window, cx)
    }
}

impl Styled for MacTextField {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// A macOS search field.
#[derive(IntoElement)]
pub struct MacSearchField {
    inner: MacTextField,
    state: Entity<InputState>,
}

impl MacSearchField {
    /// Creates a search field driven by the given input state.
    pub fn new(id: impl Into<ElementId>, state: &Entity<InputState>) -> Self {
        let state = state.clone();
        Self {
            inner: MacTextField::new(id, &state).pill(),
            state,
        }
    }

    /// Sets the AppKit control size.
    pub fn size(self, size: MacControlSize) -> Self {
        Self {
            inner: self.inner.size(size),
            ..self
        }
    }

    /// Disables the field.
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            inner: self.inner.disabled(disabled),
            ..self
        }
    }
}
impl Styled for MacSearchField {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl RenderOnce for MacSearchField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let icon_size = match self.inner.size {
            MacControlSize::Mini => 13.,
            _ => 15.,
        };
        let has_value = !self.state.read(cx).value().is_empty();
        let clear = self.state.clone();
        let inner = self.inner.prefix(magnifier(theme.label_secondary, icon_size));
        let inner = if has_value {
            inner.suffix(
                div()
                    .id("clear")
                    .on_click(move |_, window, cx| {
                        clear.update(cx, |state, cx| state.clean(window, cx));
                    })
                    .child(close_x(theme.label_secondary, icon_size - 2.)),
            )
        } else {
            inner
        };
        inner.render_field(window, cx)
    }
}

impl Styled for MacSecureField {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

/// A macOS secure field; the input state should be created masked.
#[derive(IntoElement)]
pub struct MacSecureField {
    inner: MacTextField,
}

impl MacSecureField {
    /// Creates a secure field driven by the given masked input state.
    pub fn new(id: impl Into<ElementId>, state: &Entity<InputState>) -> Self {
        Self {
            inner: MacTextField::new(id, state),
        }
    }

    /// Sets the AppKit control size.
    pub fn size(self, size: MacControlSize) -> Self {
        Self {
            inner: self.inner.size(size),
        }
    }

    /// Disables the field.
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            inner: self.inner.disabled(disabled),
        }
    }
}

impl RenderOnce for MacSecureField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.inner.render_field(window, cx)
    }
}
