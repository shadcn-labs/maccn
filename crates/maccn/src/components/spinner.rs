//! A macOS spinner.

use std::time::Duration;

use gpui::{
    App, Bounds, Element, ElementId, GlobalElementId, Hitbox, InspectorElementId, IntoElement,
    LayoutId, Pixels, RenderOnce, Window, Animation, AnimationExt as _, Img,
    ImgLayoutState, Hsla,
};

use crate::{spinner_img, theme::ThemeExt as _};

/// The two spinner sizes macvue ships.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpinnerSize {
    #[default]
    Regular,
    Small,
}

/// An element that rebuilds the spinner SVG for a new rotation angle.
struct SpinnerElement {
    id: ElementId,
    color: Hsla,
    size: f32,
    angle: f32,
    inner: Option<Img>,
}

impl SpinnerElement {
    fn new(id: ElementId, color: Hsla, size: f32) -> Self {
        Self {
            id,
            color,
            size,
            angle: 0.,
            inner: None,
        }
    }

    fn with_angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self.inner = None;
        self
    }

    fn build(&self) -> Img {
        spinner_img(self.color, self.size, self.angle)
    }
}

impl IntoElement for SpinnerElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for SpinnerElement {
    type RequestLayoutState = ImgLayoutState;
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut inner = self.inner.take().unwrap_or_else(|| self.build());
        let result = inner.request_layout(global_id, inspector_id, window, cx);
        self.inner = Some(inner);
        result
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if let Some(inner) = &mut self.inner {
            inner.prepaint(global_id, inspector_id, bounds, request_layout, window, cx)
        } else {
            None
        }
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(inner) = &mut self.inner {
            inner.paint(global_id, inspector_id, bounds, request_layout, prepaint, window, cx)
        }
    }
}

/// A macOS spinner carrying AppKit blade geometry.
#[derive(IntoElement)]
pub struct MacSpinner {
    id: ElementId,
    size: SpinnerSize,
    color: Option<Hsla>,
}

impl MacSpinner {
    /// Creates a spinner with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: SpinnerSize::Regular,
            color: None,
        }
    }

    /// Sets the size (regular 32, small 16).
    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    /// Overrides the blade color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for MacSpinner {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let size = match self.size {
            SpinnerSize::Regular => 32.,
            SpinnerSize::Small => 16.,
        };
        let color = self.color.unwrap_or(theme.spinner_color);
        let element = SpinnerElement::new(self.id.clone(), color, size);
        element.with_animation(
            (self.id.clone(), "maccn-spinner"),
            Animation::new(Duration::from_millis(800)).repeat(),
            |element, delta| element.with_angle(delta * 360.),
        )
    }
}
