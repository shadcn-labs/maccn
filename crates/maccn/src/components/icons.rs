//! Vector glyphs used by maccn controls, rendered as inline SVG images.

use gpui::{Hsla, Image, ImageFormat, IntoElement, Styled, img, px};
use std::sync::Arc;

use crate::color_hex;

fn svg(bytes: impl Into<Vec<u8>>) -> Arc<Image> {
    Arc::new(Image::from_bytes(ImageFormat::Svg, bytes.into()))
}

fn stroke_svg(body: &str, size: u32, color: Hsla) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" fill="none">{body}</svg>"#,
        body = body.replace("{color}", &color_hex(color)),
    )
}

fn svg_exact(body: &str, view_w: f32, view_h: f32, color: Hsla) -> Arc<Image> {
    svg(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {view_w} {view_h}" fill="none">{body}</svg>"#,
        body = body.replace("{color}", &color_hex(color)),
    ))
}

/// A check mark painted in the given color.
pub fn check_mark(color: Hsla, width: f32, height: f32) -> impl IntoElement {
    let body = r#"<path d="M0.9 5.1 3.3 7.8 8.4 1.1" stroke="{color}" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"/>"#;
    img(svg_exact(body, 9.3, 8.9, color))
        .w(px(width))
        .h(px(height))
}

/// A small check mark used inside pop-up menu items.
pub fn menu_check_mark(color: Hsla, width: f32, height: f32) -> impl IntoElement {
    let body = r#"<path d="M1 4.5 4 7.5 9 1.5" stroke="{color}" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>"#;
    img(svg_exact(body, 10.0, 8.0, color))
        .w(px(width))
        .h(px(height))
}

/// A horizontal dash used by indeterminate checkboxes.
pub fn dash_mark(color: Hsla, width: f32, height: f32) -> impl IntoElement {
    let body = r#"<path d="M1 1h4.5" stroke="{color}" stroke-width="2" stroke-linecap="round"/>"#;
    img(svg_exact(body, 6.5, 2.0, color))
        .w(px(width))
        .h(px(height))
}

/// A chevron pointing down.
pub fn chevron_down(color: Hsla, size: f32) -> impl IntoElement {
    let body = r#"<path d="M4 6l4 4 4-4" stroke="{color}" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/>"#;
    img(svg(stroke_svg(body, 16, color))).size(px(size))
}

/// A chevron pointing up.
pub fn chevron_up(color: Hsla, size: f32) -> impl IntoElement {
    let body = r#"<path d="M4 10l4-4 4 4" stroke="{color}" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/>"#;
    img(svg(stroke_svg(body, 16, color))).size(px(size))
}

/// The up/down chevron used by pop-up buttons.
pub fn pop_up_chevron(color: Hsla, width: f32, height: f32) -> impl IntoElement {
    let body = r#"<path d="M1 4 4 1 7 4M1 8 4 11 7 8" stroke="{color}" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>"#;
    img(svg_exact(body, 8.0, 12.0, color))
        .w(px(width))
        .h(px(height))
}

/// The small up chevron used by steppers.
pub fn stepper_chevron_up(color: Hsla, width: f32, height: f32) -> impl IntoElement {
    let body = r#"<path d="M1 5 5.3 1 9.6 5" stroke="{color}" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"/>"#;
    img(svg_exact(body, 10.6, 6.0, color))
        .w(px(width))
        .h(px(height))
}

/// The small down chevron used by steppers.
pub fn stepper_chevron_down(color: Hsla, width: f32, height: f32) -> impl IntoElement {
    let body = r#"<path d="M1 1 5.3 5 9.6 1" stroke="{color}" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"/>"#;
    img(svg_exact(body, 10.6, 6.0, color))
        .w(px(width))
        .h(px(height))
}

/// A magnifier used by search fields.
pub fn magnifier(color: Hsla, size: f32) -> impl IntoElement {
    let body = concat!(
        r#"<circle cx="6.5" cy="6.5" r="4.5" stroke="{color}" stroke-width="1.6"/>"#,
        r#"<path d="M10 10 14 14" stroke="{color}" stroke-width="1.6" stroke-linecap="round"/>"#,
    );
    img(svg(stroke_svg(body, 16, color))).size(px(size))
}

/// A close glyph used by search fields.
pub fn close_x(color: Hsla, size: f32) -> impl IntoElement {
    let body = r#"<path d="M5 5l6 6M11 5l-6 6" stroke="{color}" stroke-width="1.6" stroke-linecap="round"/>"#;
    img(svg(stroke_svg(body, 16, color))).size(px(size))
}

/// A spinner composed of eight blades; the caller animates its angle.
pub fn spinner_svg(color: Hsla, size: f32, angle_deg: f32) -> impl IntoElement {
    spinner_img(color, size, angle_deg)
}

/// The raw image element used by [`spinner_svg`] and the animated spinner.
pub fn spinner_img(color: Hsla, size: f32, angle_deg: f32) -> gpui::Img {
    let mut blades = String::new();
    for i in 0..8 {
        let rotation = i as f32 * 45. + angle_deg;
        let opacity = (55. - i as f32 * 7.) / 100.;
        blades.push_str(&format!(
            r#"<rect x="14" y="0" width="4" height="10" rx="2" fill="{color}" opacity="{opacity}" transform="rotate({rotation} 16 16)"/>"#,
            color = color_hex(color),
        ));
    }
    img(svg(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32">{blades}</svg>"#
    )))
    .size(px(size))
}
