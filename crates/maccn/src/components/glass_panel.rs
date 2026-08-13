//! A macOS Liquid-Glass-inspired panel.

use gpui::{
    App, AnyElement, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::{MaccnAppearance, rgba_f, theme::ThemeExt as _};

/// The glass materials macvue ships.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GlassMaterial {
    #[default]
    Regular,
    Clear,
}

/// A translucent panel with a subtle rim.
///
/// GPUI has no backdrop blur, so the translucent fill is approximated with the
/// material's flat tint.
#[derive(IntoElement)]
pub struct MacGlassPanel {
    inner: gpui::Stateful<gpui::Div>,
    material: GlassMaterial,
}

impl MacGlassPanel {
    /// Creates a glass panel with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            inner: div().id(id.clone()),
            material: GlassMaterial::Regular,
        }
    }

    /// Sets the material.
    pub fn material(mut self, material: GlassMaterial) -> Self {
        self.material = material;
        self
    }
}

impl Styled for MacGlassPanel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for MacGlassPanel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for MacGlassPanel {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for MacGlassPanel {}

impl RenderOnce for MacGlassPanel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let light = theme.appearance == MaccnAppearance::Light;
        let (bg, rim): (Hsla, Hsla) = match (light, self.material) {
            (true, GlassMaterial::Regular) => (rgba_f(255, 255, 255, 0.7), rgba_f(0, 0, 0, 0.2)),
            (true, GlassMaterial::Clear) => (rgba_f(255, 255, 255, 0.28), rgba_f(0, 0, 0, 0.18)),
            (false, GlassMaterial::Regular) => {
                (rgba_f(0, 0, 0, 0.32), rgba_f(255, 255, 255, 0.1))
            }
            (false, GlassMaterial::Clear) => {
                (rgba_f(0, 0, 0, 0.12), rgba_f(255, 255, 255, 0.14))
            }
        };
        self.inner
            .flex()
            .flex_col()
            .overflow_hidden()
            .p(px(16.))
            .rounded(px(24.))
            .bg(bg)
            .border_1()
            .border_color(rim)
    }
}
