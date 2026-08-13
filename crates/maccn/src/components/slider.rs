//! A macOS slider.

use gpui::{
    App, ElementId, Entity, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window, div, prelude::FluentBuilder as _, px, relative,
};
use gpui_base::{
    Slider as BaseSlider, SliderIndicator, SliderThumb, SliderTrack, StyledExt as _,
    slider::SliderState,
};

use crate::{MacControlSize, theme::ThemeExt as _};

fn geometry(size: MacControlSize) -> (f32, f32, f32, f32) {
    match size {
        MacControlSize::ExtraLarge => (36., 6., 24., 10.),
        MacControlSize::Large => (28., 6., 24., 10.),
        MacControlSize::Regular => (24., 6., 20., 8.),
        MacControlSize::Small => (20., 4., 18., 7.),
        MacControlSize::Mini => (16., 4., 16., 7.),
    }
}

/// A macOS slider with AppKit track and knob metrics.
#[derive(IntoElement)]
pub struct MacSlider {
    state: Entity<SliderState>,
    size: MacControlSize,
    disabled: bool,
    style: gpui::StyleRefinement,
}

impl MacSlider {
    /// Creates a slider driven by the given state.
    pub fn new(_id: impl Into<ElementId>, state: &Entity<SliderState>) -> Self {
        Self {
            state: state.clone(),
            size: MacControlSize::Regular,
            disabled: false,
            style: gpui::StyleRefinement::default(),
        }
    }

    /// Sets the AppKit control size.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Disables the slider.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for MacSlider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSlider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let style = self.style;
        let size = self.size;
        let disabled = self.disabled;
        let (height, track_size, thumb_w, thumb_radius) = geometry(size);
        let thumb_h = match size {
            MacControlSize::ExtraLarge | MacControlSize::Large => 20.,
            MacControlSize::Regular => 16.,
            MacControlSize::Small => 14.,
            MacControlSize::Mini => 12.,
        };
        let percentage = self.state.read(cx).percentage().end;

        let track_y = (height - track_size) / 2.;
        let thumb_y = (height - thumb_h) / 2.;

        BaseSlider::new(&self.state)
            .disabled(disabled)
            .refine_style(&style)
            .relative()
            .w_full()
            .h(px(height))
            .child(
                SliderTrack::new(&self.state)
                    .disabled(disabled)
                    .relative()
                    .w_full()
                    .h_full()
                    .child(
                        div()
                            .absolute()
                            .top(px(track_y))
                            .left_0()
                            .w_full()
                            .h(px(track_size))
                            .rounded_full()
                            .bg(theme.control_track)
                            .shadow(crate::switch_track_shadow())
                            .when(disabled, |this| this.bg(theme.control_disabled)),
                    )
                    .child(
                        SliderIndicator::new(&self.state)
                            .absolute()
                            .top(px(track_y))
                            .left_0()
                            .w_full()
                            .h(px(track_size))
                            .child(
                                div()
                                    .absolute()
                                    .top_0()
                                    .bottom_0()
                                    .left_0()
                                    .right(relative(1. - percentage))
                                    .rounded_full()
                                    .bg(theme.accent)
                                    .when(disabled, |this| this.bg(theme.accent_disabled)),
                            ),
                    )
                    .child(
                        SliderThumb::new(&self.state)
                            .disabled(disabled)
                            .absolute()
                            .top(px(thumb_y))
                            .left(relative(percentage))
                            .ml(px(-thumb_w / 2.))
                            .w(px(thumb_w))
                            .h(px(thumb_h))
                            .rounded(px(thumb_radius))
                            .bg(theme.slider_knob)
                            .shadow(crate::switch_thumb_shadow()),
                    ),
            )
    }
}
