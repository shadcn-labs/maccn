//! maccn design tokens.
//!
//! Light and dark palettes are ported from macvue's semantic tokens, which
//! carry the AppKit `NSColor` metrics measured against the macOS Tahoe
//! reference. Every color resolves through the active [`MaccnTheme`], so a
//! single accent change re-tints every control.

use gpui::{App, Global, Hsla, rgba};

/// The two system appearances.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaccnAppearance {
    Light,
    Dark,
}

/// Converts an `RRGGBBAA` hex value to an [`Hsla`].
#[inline]
fn c(hex: u32) -> Hsla {
    rgba(hex).into()
}

/// Accent colors as defined by macvue's reference palette.
pub mod accent {
    use super::*;

    pub const BLUE: Hsla = Hsla {
        h: 0.57777778,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    };
    pub const BLUE_DARK: Hsla = Hsla {
        h: 0.57189542,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    };
    pub const PURPLE: Hsla = Hsla {
        h: 0.81344697,
        s: 0.739496,
        l: 0.533333,
        a: 1.0,
    };
    pub const PINK: Hsla = Hsla {
        h: 0.96825397,
        s: 1.0,
        l: 0.588235,
        a: 1.0,
    };
    pub const RED: Hsla = Hsla {
        h: 0.99664992,
        s: 1.0,
        l: 0.609804,
        a: 1.0,
    };
    pub const ORANGE: Hsla = Hsla {
        h: 0.07829457,
        s: 1.0,
        l: 0.578431,
        a: 1.0,
    };
    pub const YELLOW: Hsla = Hsla {
        h: 0.13333333,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    };
    pub const GREEN: Hsla = Hsla {
        h: 0.37528345,
        s: 0.585657,
        l: 0.492157,
        a: 1.0,
    };
    pub const GRAPHITE: Hsla = Hsla {
        h: 0.66666667,
        s: 0.022624,
        l: 0.566667,
        a: 1.0,
    };
}

/// The full semantic palette for a single appearance.
#[derive(Clone, Copy)]
pub struct MaccnTheme {
    pub appearance: MaccnAppearance,
    pub accent: Hsla,

    pub label: Hsla,
    pub label_secondary: Hsla,
    pub label_tertiary: Hsla,
    pub label_quaternary: Hsla,
    pub label_disabled: Hsla,
    pub label_on_accent: Hsla,
    pub label_on_accent_disabled: Hsla,

    pub window_bg: Hsla,
    pub control_bg: Hsla,
    pub control_bg_disabled: Hsla,
    pub content_bg: Hsla,
    pub under_page_bg: Hsla,
    pub group_box: Hsla,

    pub control: Hsla,
    pub control_pressed: Hsla,
    pub control_disabled: Hsla,
    pub control_disabled_strong: Hsla,
    pub control_track: Hsla,
    pub control_track_pressed: Hsla,
    pub progress_bar_track: Hsla,
    pub control_button: Hsla,
    pub control_button_pressed: Hsla,
    pub control_button_opaque: Hsla,
    pub control_button_pressed_opaque: Hsla,
    pub segmented_container: Hsla,

    pub accent_pressed: Hsla,
    pub accent_pressed_soft: Hsla,
    pub accent_disabled: Hsla,
    pub accent_disabled_strong: Hsla,
    pub segmented_on_pressed: Hsla,
    pub focus_ring: Hsla,
    pub focus_ring_border: Hsla,
    pub selection_bg: Hsla,
    pub selection_bg_unfocused: Hsla,

    pub destructive: Hsla,
    pub destructive_bg: Hsla,
    pub destructive_bg_pressed: Hsla,

    pub separator: Hsla,
    pub separator_on_control: Hsla,
    pub separator_vibrant: Hsla,
    pub border_control: Hsla,

    pub switch_thumb: Hsla,
    pub slider_knob: Hsla,
    pub spinner_color: Hsla,

    pub menu_bg: Hsla,
    pub menu_rim: Hsla,
}

impl Global for MaccnTheme {}

impl MaccnTheme {
    pub fn light() -> Self {
        Self {
            appearance: MaccnAppearance::Light,
            accent: accent::BLUE,

            label: c(0x000000D9),
            label_secondary: c(0x00000080),
            label_tertiary: c(0x00000040),
            label_quaternary: c(0x0000001A),
            label_disabled: c(0xBFBFBFFF),
            label_on_accent: c(0xFFFFFFFF),
            label_on_accent_disabled: c(0xFFFFFFFF),

            window_bg: c(0xF6F6F6FF),
            control_bg: c(0xFFFFFFFF),
            control_bg_disabled: c(0xFFFFFF80),
            content_bg: c(0xFFFFFFFF),
            under_page_bg: c(0xE8E8E8FF),
            group_box: c(0x00000008),

            control: c(0x0000000D),
            control_pressed: c(0x00000026),
            control_disabled: c(0x0000000A),
            control_disabled_strong: c(0x00000014),
            control_track: c(0x0000000F),
            control_track_pressed: c(0x00000022),
            progress_bar_track: c(0x0000000F),
            control_button: c(0x0000000D),
            control_button_pressed: c(0x00000026),
            control_button_opaque: c(0xF2F2F2FF),
            control_button_pressed_opaque: c(0xD9D9D9FF),
            segmented_container: c(0x0000000D),

            accent_pressed: c(0x0074D9FF),
            accent_pressed_soft: c(0x007DEBFF),
            accent_disabled: c(0x0088FF40),
            accent_disabled_strong: c(0x0088FF80),
            segmented_on_pressed: c(0x0074D9FF),
            focus_ring: c(0x0088FF40),
            focus_ring_border: c(0x0088FF26),
            selection_bg: c(0x0088FFFF),
            selection_bg_unfocused: c(0x0000001A),

            destructive: c(0xFF383CFF),
            destructive_bg: c(0xFF383C40),
            destructive_bg_pressed: c(0xD930335C),

            separator: c(0x0000001A),
            separator_on_control: c(0xE6E6E6FF),
            separator_vibrant: c(0xBFBFBFFF),
            border_control: c(0x00000014),

            switch_thumb: c(0xFFFFFFDE),
            slider_knob: c(0xFFFFFFFF),
            spinner_color: c(0x000000FF),

            menu_bg: c(0xF6F6F6F0),
            menu_rim: c(0x0000002E),
        }
    }

    pub fn dark() -> Self {
        Self {
            appearance: MaccnAppearance::Dark,
            accent: accent::BLUE_DARK,

            label: c(0xFFFFFFFF),
            label_secondary: c(0xFFFFFF8C),
            label_tertiary: c(0xFFFFFF40),
            label_quaternary: c(0xFFFFFF1A),
            label_disabled: c(0xFFFFFF40),
            label_on_accent: c(0xFFFFFFFF),
            label_on_accent_disabled: c(0xFFFFFF80),

            window_bg: c(0x1E1E1EFF),
            control_bg: c(0x1E1E1EFF),
            control_bg_disabled: c(0x1E1E1E80),
            content_bg: c(0x272727FF),
            under_page_bg: c(0x282828FF),
            group_box: c(0xFFFFFF08),

            control: c(0xFFFFFF1A),
            control_pressed: c(0xFFFFFF30),
            control_disabled: c(0xFFFFFF0A),
            control_disabled_strong: c(0xFFFFFF1A),
            control_track: c(0xFFFFFF1A),
            control_track_pressed: c(0xFFFFFF2C),
            progress_bar_track: c(0xFFFFFF14),
            control_button: c(0xFFFFFF12),
            control_button_pressed: c(0xFFFFFF29),
            control_button_opaque: c(0x2D2D2DFF),
            control_button_pressed_opaque: c(0x444444FF),
            segmented_container: c(0xFFFFFF14),

            accent_pressed: c(0x149AFFFF),
            accent_pressed_soft: c(0x1A9CFFFF),
            accent_disabled: c(0x0091FF66),
            accent_disabled_strong: c(0x0091FFE6),
            segmented_on_pressed: c(0x24A0FFFF),
            focus_ring: c(0x0091FF40),
            focus_ring_border: c(0x0091FF26),
            selection_bg: c(0x0091FFFF),
            selection_bg_unfocused: c(0xFFFFFF1A),

            destructive: c(0xFF4245FF),
            destructive_bg: c(0xFF424533),
            destructive_bg_pressed: c(0xFF424547),

            separator: c(0xFFFFFF1A),
            separator_on_control: c(0x262626FF),
            separator_vibrant: c(0x262626FF),
            border_control: c(0xFFFFFF0A),

            switch_thumb: c(0xFFFFFFD9),
            slider_knob: c(0xDEDEDEFF),
            spinner_color: c(0xFFFFFFFF),

            menu_bg: c(0x2E2E2EF5),
            menu_rim: c(0xFFFFFF29),
        }
    }

    /// Returns the active theme, defaulting to light.
    pub fn global(cx: &App) -> Self {
        cx.try_global::<Self>().copied().unwrap_or_else(Self::light)
    }

    /// Returns the active theme, inserting the light default if absent.
    pub fn global_mut(cx: &mut App) -> &mut Self {
        if !cx.has_global::<Self>() {
            cx.set_global(Self::light());
        }
        cx.global_mut::<Self>()
    }
}

/// Convenience access to the active theme through an application context.
pub trait ThemeExt {
    fn theme(&self) -> MaccnTheme;
}

impl ThemeExt for App {
    #[inline(always)]
    fn theme(&self) -> MaccnTheme {
        MaccnTheme::global(self)
    }
}

/// Initializes the default maccn theme.
pub fn init(cx: &mut App) {
    if !cx.has_global::<MaccnTheme>() {
        cx.set_global(MaccnTheme::light());
    }
}
