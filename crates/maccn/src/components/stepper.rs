//! A macOS stepper.

use std::rc::Rc;

use gpui::{
    App, AnyElement, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_base::Button as BaseButton;

use crate::{MacControlSize, control_height, field_radius, stepper_chevron_down, stepper_chevron_up, theme::ThemeExt as _};

type StepHandler = Rc<dyn Fn(&mut Window, &mut App)>;

fn geometry(size: MacControlSize) -> (f32, f32, f32, f32) {
    match size {
        MacControlSize::ExtraLarge => (30., 20., 10.6, 6.),
        MacControlSize::Large => (23., 15., 10.6, 6.),
        MacControlSize::Regular => (20., 14., 10.6, 6.),
        MacControlSize::Small => (17., 11., 8., 4.6),
        MacControlSize::Mini => (13., 9., 6.8, 3.8),
    }
}

/// A macOS stepper with increment and decrement buttons.
#[derive(IntoElement)]
pub struct MacStepper {
    inner: gpui::Stateful<gpui::Div>,
    id: ElementId,
    size: MacControlSize,
    disabled: bool,
    on_increment: Option<StepHandler>,
    on_decrement: Option<StepHandler>,
}

impl MacStepper {
    /// Creates a stepper with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: div().id(id.clone()),
            id,
            size: MacControlSize::Regular,
            disabled: false,
            on_increment: None,
            on_decrement: None,
        }
    }

    /// Sets the AppKit control size.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Disables the stepper.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the increment handler.
    pub fn on_increment(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_increment = Some(Rc::new(handler));
        self
    }

    /// Sets the decrement handler.
    pub fn on_decrement(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_decrement = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacStepper {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacStepper {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacStepper {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacStepper {}

impl RenderOnce for MacStepper {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let size = self.size;
        let disabled = self.disabled;
        let height = control_height(size);
        let (width, separator_width, chevron_w, chevron_h) = geometry(size);
        let on_increment = self.on_increment;
        let on_decrement = self.on_decrement;

        let chevron_color = if disabled {
            theme.label_tertiary
        } else {
            theme.label
        };

        let up = BaseButton::new((self.id.clone(), "increment"))
            .disabled(disabled)
            .focusable(false)
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .when_some(on_increment, |button, handler| {
                button.on_click(move |_: &ClickEvent, window, cx| handler(window, cx))
            })
            .active(|style| style.bg(theme.control_pressed))
            .child(stepper_chevron_up(chevron_color, chevron_w, chevron_h));

        let down = BaseButton::new((self.id.clone(), "decrement"))
            .disabled(disabled)
            .focusable(false)
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .when_some(on_decrement, |button, handler| {
                button.on_click(move |_: &ClickEvent, window, cx| handler(window, cx))
            })
            .active(|style| style.bg(theme.control_pressed))
            .child(stepper_chevron_down(chevron_color, chevron_w, chevron_h));

        self.inner
            .flex()
            .flex_none()
            .flex_col()
            .items_center()
            .w(px(width))
            .h(px(height))
            .rounded(px(field_radius(size)))
            .overflow_hidden()
            .bg(theme.control)
            .when(disabled, |this| this.bg(theme.control_disabled))
            .child(up)
            .child(
                div()
                    .flex_none()
                    .w(px(separator_width))
                    .h(px(1.))
                    .bg(theme.separator_vibrant),
            )
            .child(down)
    }
}
