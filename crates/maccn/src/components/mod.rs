//! macOS-inspired components built on [`gpui-base`] primitives.
//!
//! Every control mirrors macvue's AppKit metrics: the five
//! [`MacControlSize`]s, light and dark palettes, and the system accent.

mod badge;
mod box_;
mod button;
mod checkbox;
mod glass_panel;
mod help_button;
mod icons;
mod label;
mod pop_up_button;
mod progress;
mod radio;
mod segmented_control;
mod separator;
mod slider;
mod spinner;
mod stepper;
mod switch;
mod text_field;

pub use badge::MacBadge;
pub use box_::MacBox;
pub use button::{ButtonVariant, MacButton};
pub use checkbox::{MacCheckbox, MacCheckboxState};
pub use glass_panel::{GlassMaterial, MacGlassPanel};
pub use help_button::MacHelpButton;
pub use icons::*;
pub use label::{LabelStyle, MacLabel};
pub use pop_up_button::{MacPopUpButton, MacPopUpButtonItem};
pub use progress::{MacProgress, ProgressSize};
pub use radio::{MacRadio, MacRadioGroup};
pub use segmented_control::{MacSegment, MacSegmentedControl};
pub use separator::MacSeparator;
pub use slider::MacSlider;
pub use spinner::{MacSpinner, SpinnerSize};
pub use stepper::MacStepper;
pub use switch::MacSwitch;
pub use text_field::{MacSearchField, MacSecureField, MacTextField};

use crate::MacControlSize;
use gpui::{BoxShadow, FontWeight, Hsla, rgba, px};

/// A radius large enough to read as a pill.
pub const PILL: f32 = 1000.;

/// AppKit `NSControl.ControlSize` heights.
pub fn control_height(size: MacControlSize) -> f32 {
    match size {
        MacControlSize::ExtraLarge => 36.,
        MacControlSize::Large => 28.,
        MacControlSize::Regular => 24.,
        MacControlSize::Small => 20.,
        MacControlSize::Mini => 16.,
    }
}

/// Border radius of push-button-like controls.
pub fn control_radius(size: MacControlSize) -> f32 {
    match size {
        MacControlSize::ExtraLarge | MacControlSize::Large => PILL,
        MacControlSize::Regular => 6.,
        MacControlSize::Small => 5.,
        MacControlSize::Mini => 4.,
    }
}

/// Border radius of input fields and steppers.
pub fn field_radius(size: MacControlSize) -> f32 {
    match size {
        MacControlSize::ExtraLarge => 9.,
        MacControlSize::Large => 7.,
        MacControlSize::Regular => 6.,
        MacControlSize::Small => 5.,
        MacControlSize::Mini => 4.,
    }
}

/// Text size of controls for the given size.
pub fn control_text_size(size: MacControlSize) -> f32 {
    match size {
        MacControlSize::ExtraLarge | MacControlSize::Large | MacControlSize::Regular => 13.,
        MacControlSize::Small => 11.,
        MacControlSize::Mini => 10.,
    }
}

/// The weight macvue applies to control labels.
pub const CONTROL_TEXT_WEIGHT: FontWeight = FontWeight::MEDIUM;

/// Opacity applied to disabled controls that do not use exact fills.
pub const DISABLED_OPACITY: f32 = 0.5;

/// The kit's focused-field accent ring, expressed as a glow.
pub fn focus_ring_shadow(color: Hsla) -> Vec<BoxShadow> {
    vec![gpui_base::box_shadow(px(0.), px(0.), px(3.5), px(0.5), color)]
}

/// Converts a color to a `#rrggbb` hex string for SVG paints.
pub fn color_hex(color: Hsla) -> String {
    let rgb = color.to_rgb();
    format!(
        "#{:02x}{:02x}{:02x}",
        (rgb.r * 255.) as u8,
        (rgb.g * 255.) as u8,
        (rgb.b * 255.) as u8
    )
}

/// The hairline ring macvue draws around the unfilled switch track.
pub fn switch_track_shadow() -> Vec<BoxShadow> {
    vec![gpui_base::box_shadow(
        px(0.),
        px(0.),
        px(1.),
        px(0.),
        rgba(0x0000000F).into(),
    )]
}

/// The knob shadow macvue draws under the switch thumb and slider knob.
pub fn switch_thumb_shadow() -> Vec<BoxShadow> {
    vec![gpui_base::box_shadow(
        px(0.),
        px(2.),
        px(4.),
        px(0.),
        rgba(0x00000026).into(),
    )]
}

/// Builds an [`Hsla`] from RGBA byte channels.
pub fn rgba_f(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    rgba((r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | (a * 255.) as u32).into()
}
