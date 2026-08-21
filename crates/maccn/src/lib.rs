//! maccn — macOS-inspired controls built for GPUI on top of [`gpui-base`].
//!
//! The components carry the AppKit metrics, states and keyboard behavior
//! captured by [macvue](https://github.com/antonreshetov/macvue), re-expressed
//! as composable GPUI components. Every control supports the five
//! `NSControl.ControlSize` sizes, light and dark appearances, and a system
//! accent color.

pub mod components;
pub mod sizes;
pub mod theme;

pub use components::*;
pub use sizes::MacControlSize;
pub use theme::{MaccnAppearance, MaccnTheme, ThemeExt};

// Re-export gpui-base traits for convenience.
pub use gpui_base::{Disableable, FocusableExt, RoleOverride, Selectable};

use gpui::App;

/// Initializes global infrastructure owned by maccn.
pub fn init(cx: &mut App) {
    gpui_base::init(cx);
    theme::init(cx);
}
