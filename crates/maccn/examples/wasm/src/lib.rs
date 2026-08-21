#[cfg(target_family = "wasm")]
use gpui::{Application, ApplicationHandle};
#[cfg(target_family = "wasm")]
use std::cell::RefCell;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[path = "../../showcase/mod.rs"]
#[allow(dead_code)]
mod showcase;

#[cfg(target_family = "wasm")]
thread_local! {
    static APPLICATION: RefCell<Option<ApplicationHandle>> = const { RefCell::new(None) };
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn run(component: Option<String>, card_mode: Option<bool>, dark: Option<bool>) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);
    tracing_wasm::set_as_global_default();
    gpui_platform::web_init();

    let component = component.unwrap_or_else(|| "overview".to_owned());
    let card_mode = card_mode.unwrap_or(false);

    let handle = showcase::run_embedded(web_application(), component, card_mode, dark);
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
    Ok(())
}

/// Switches the running gallery between light and dark after it is live.
///
/// The embedding page calls this when its own appearance changes so the
/// WASM demo never sits in a dark page wearing a light theme (or vice versa).
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn set_theme(dark: bool) {
    use maccn::{MaccnAppearance, MaccnTheme};

    APPLICATION.with(|application| {
        if let Some(handle) = application.borrow().as_ref() {
            handle.update(|cx| {
                let appearance = if dark {
                    MaccnAppearance::Dark
                } else {
                    MaccnAppearance::Light
                };
                // Preserve the current accent when switching light ↔ dark.
                let current_accent = MaccnTheme::global(cx).accent;
                let new_theme = match appearance {
                    MaccnAppearance::Light => MaccnTheme::light(),
                    MaccnAppearance::Dark => MaccnTheme::dark(),
                };
                cx.set_global(new_theme.with_accent(current_accent));
                cx.refresh_windows();
            });
        }
    });
}

/// Updates the system accent colour for every running component.
///
/// `hex` is an `0xAARRGGBB` or `0xRRGGBB` value (matching GPUI's `rgba()`
/// convention).  All accent-derived tokens (pressed, disabled, focus ring, …)
/// are recomputed automatically.
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn set_accent(hex: u32) {
    use maccn::MaccnTheme;

    let hsla: gpui::Hsla = gpui::rgba(hex).into();
    log::info!("[maccn] set_accent hex={:#010x} -> hsla=({:.3},{:.3},{:.3},{:.3})", hex, hsla.h, hsla.s, hsla.l, hsla.a);
    APPLICATION.with(|application| {
        if let Some(handle) = application.borrow().as_ref() {
            handle.update(|cx| {
                let current = MaccnTheme::global(cx);
                cx.set_global(current.with_accent(hsla));
                cx.refresh_windows();
            });
        }
    });
}

#[cfg(target_family = "wasm")]
fn web_application() -> Application {
    gpui_platform::single_threaded_web()
}
