//! A macOS switch.

use gpui::{
    App, AnyElement, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_base::{Switch as BaseSwitch, SwitchThumb, SwitchTrack};

use crate::{MacControlSize, control_text_size, theme::ThemeExt as _, CONTROL_TEXT_WEIGHT};

fn track_size(size: MacControlSize) -> (f32, f32) {
    match size {
        MacControlSize::ExtraLarge => (80., 36.),
        MacControlSize::Large => (64., 28.),
        MacControlSize::Regular => (54., 24.),
        MacControlSize::Small => (44., 20.),
        MacControlSize::Mini => (36., 16.),
    }
}

fn thumb_size(size: MacControlSize) -> (f32, f32) {
    match size {
        MacControlSize::ExtraLarge => (47., 30.),
        MacControlSize::Large => (38., 24.),
        MacControlSize::Regular => (32., 20.),
        MacControlSize::Small => (26., 16.),
        MacControlSize::Mini => (21., 13.),
    }
}

fn thumb_inset(size: MacControlSize) -> f32 {
    match size {
        MacControlSize::ExtraLarge => 3.,
        MacControlSize::Large | MacControlSize::Regular | MacControlSize::Small => 2.,
        MacControlSize::Mini => 1.5,
    }
}

/// A macOS switch carrying AppKit track metrics.
#[derive(IntoElement)]
pub struct MacSwitch {
    inner: BaseSwitch,
    id: ElementId,
    size: MacControlSize,
    checked: bool,
    disabled: bool,
    children: Vec<AnyElement>,
}

impl MacSwitch {
    /// Creates a switch with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: BaseSwitch::new(id.clone()),
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

    /// Sets the application-controlled checked value.
    pub fn checked(self, checked: bool) -> Self {
        Self {
            inner: self.inner.checked(checked),
            checked,
            ..self
        }
    }

    /// Disables the switch.
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            inner: self.inner.disabled(disabled),
            disabled,
            ..self
        }
    }

    /// Handles activation with the next checked value.
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

impl Styled for MacSwitch {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacSwitch {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl InteractiveElement for MacSwitch {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacSwitch {}

impl RenderOnce for MacSwitch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let size = self.size;
        let checked = self.checked;
        let disabled = self.disabled;
        let (track_w, track_h) = track_size(size);
        let (thumb_w, thumb_h) = thumb_size(size);
        let inset = thumb_inset(size);
        let travel = track_w - thumb_w - 2. * inset;

        let track = SwitchTrack::new((self.id.clone(), "track"))
            .checked(checked)
            .disabled(disabled)
            .flex_none()
            .items_center()
            .w(px(track_w))
            .h(px(track_h))
            .p(px(inset))
            .rounded_full()
            .bg(theme.control_track)
            .shadow(crate::switch_track_shadow())
            .styles(|styles| {
                styles
                    .checked(|style| style.bg(theme.accent))
                    .disabled(|style| {
                        style.when(checked, |style| style.bg(theme.accent_disabled_strong))
                    })
            })
            .active(|style| {
                style.bg(if checked {
                    theme.accent_pressed_soft
                } else {
                    theme.control_track_pressed
                })
            })
            .child(
                SwitchThumb::new(checked)
                    .disabled(disabled)
                    .flex_none()
                    .w(px(thumb_w))
                    .h(px(thumb_h))
                    .rounded_full()
                    .bg(theme.switch_thumb)
                    .shadow(crate::switch_thumb_shadow())
                    .when(checked, |thumb| thumb.ml(px(travel))),
            );

        self.inner
            .flex()
            .items_center()
            .gap(px(8.))
            .text_size(px(control_text_size(size)))
            .font_weight(CONTROL_TEXT_WEIGHT)
            .text_color(theme.label)
            .child(div().flex_none().children(self.children))
            .child(track)
    }
}
