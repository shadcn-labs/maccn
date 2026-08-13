//! A macOS linear progress bar.

use std::time::Duration;

use gpui::{
    App, ElementId, IntoElement, ParentElement, RenderOnce, Styled,
    Window, div, px, relative, Animation, AnimationExt as _,
};
use gpui_base::{Progress as BaseProgress, StyledExt as _};

use crate::theme::ThemeExt as _;

/// The two progress bar heights.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProgressSize {
    #[default]
    Regular,
    Small,
}

/// A macOS progress bar.
#[derive(IntoElement)]
pub struct MacProgress {
    inner: BaseProgress,
    id: ElementId,
    size: ProgressSize,
    value: f32,
    indeterminate: bool,
    style: gpui::StyleRefinement,
}

impl MacProgress {
    /// Creates a progress bar with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: BaseProgress::new(id.clone()),
            id,
            size: ProgressSize::Regular,
            value: 0.,
            indeterminate: false,
            style: gpui::StyleRefinement::default(),
        }
    }

    /// Sets the bar height.
    pub fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the controlled percentage value.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    /// Switches to the indeterminate busy state.
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }
}

impl Styled for MacProgress {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacProgress {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let style = self.style;
        let height = match self.size {
            ProgressSize::Regular => 10.,
            ProgressSize::Small => 6.,
        };
        let value = self.value;
        let indeterminate = self.indeterminate;

        let indicator = if indeterminate {
            let bar = div()
                .absolute()
                .top_0()
                .bottom_0()
                .w(relative(0.4))
                .bg(theme.accent)
                .rounded_full();
            div()
                .absolute()
                .top_0()
                .bottom_0()
                .left_0()
                .w_full()
                .overflow_hidden()
                .child(
                    bar.with_animation(
                        (self.id.clone(), "maccn-progress-sweep"),
                        Animation::new(Duration::from_millis(1400)).repeat(),
                        move |element, delta| element.left(relative(delta * 1.4 - 0.4)),
                    ),
                )
        } else {
            div()
                .absolute()
                .top_0()
                .bottom_0()
                .left_0()
                .w(relative((value / 100.).clamp(0., 1.)))
                .bg(theme.accent)
                .rounded_full()
        };

        self.inner
            .value(value)
            .indeterminate(indeterminate)
            .refine_style(&style)
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(height))
                    .rounded_full()
                    .bg(theme.progress_bar_track)
                    .shadow(crate::switch_track_shadow())
                    .child(indicator),
            )
    }
}
