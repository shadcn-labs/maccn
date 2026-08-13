#![allow(dead_code)]

//! The interactive component showcase shared by the native example and the
//! WASM web demo. Included through `#[path]`, mirroring gpui-component's
//! `crates/base/examples/showcase`.

use gpui::{
    App, AppContext as _, Application, Context, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, ScrollHandle, StatefulInteractiveElement as _, Styled as _, Window,
    WindowOptions, actions, div, prelude::FluentBuilder as _, px, size,
};
#[cfg(not(target_family = "wasm"))]
use gpui::{KeyBinding, WindowBounds};
use gpui_base::input::{InputEvent, InputState};
use gpui_base::slider::SliderState;

use maccn::{
    MacButton, MacControlSize, MaccnAppearance, MaccnTheme, ThemeExt as _,
};

mod components;

actions!(maccn_showcase, [Quit]);

pub const COMPONENTS: &[&str] = &[
    "badge",
    "box",
    "button",
    "checkbox",
    "glass-panel",
    "help-button",
    "label",
    "pop-up-button",
    "progress",
    "radio-group",
    "search-field",
    "secure-field",
    "segmented-control",
    "separator",
    "slider",
    "spinner",
    "stepper",
    "switch",
    "text-field",
];

pub struct Showcase {
    component: String,
    navigation_enabled: bool,
    appearance: MaccnAppearance,
    checkbox_checked: bool,
    radio_selected: usize,
    switch_checked: bool,
    toggle_checked: bool,
    segmented_index: usize,
    stepper_value: i32,
    popup_selected: Option<usize>,
    popup_open: bool,
    progress_value: f32,
    slider: Entity<SliderState>,
    input: Entity<InputState>,
    secure_input: Entity<InputState>,
    search_input: Entity<InputState>,
    scroll: ScrollHandle,
}

impl Showcase {
    pub fn new(component: impl Into<String>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let component = component.into();
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Placeholder text")
                .default_value("")
        });
        let secure_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Password")
                .default_value("hunter2")
                .masked(true)
        });
        let search_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Search")
                .default_value("")
        });
        for state in [&input, &secure_input, &search_input] {
            cx.subscribe(state, |_, _, _: &InputEvent, cx| cx.notify())
                .detach();
        }

        let slider = cx.new(|_| SliderState::new().min(0.).max(100.).default_value(64.));
        cx.observe(&slider, |_, _, cx| cx.notify()).detach();

        let appearance = MaccnTheme::global(cx).appearance;

        Self {
            navigation_enabled: component == "overview",
            component,
            appearance,
            checkbox_checked: true,
            radio_selected: 0,
            switch_checked: true,
            toggle_checked: true,
            segmented_index: 0,
            stepper_value: 0,
            popup_selected: None,
            popup_open: false,
            progress_value: 0.68,
            slider,
            input,
            secure_input,
            search_input,
            scroll: ScrollHandle::new(),
        }
    }

    fn set_appearance(&mut self, appearance: MaccnAppearance, cx: &mut Context<Self>) {
        if self.appearance == appearance {
            return;
        }
        self.appearance = appearance;
        cx.set_global(match appearance {
            MaccnAppearance::Light => MaccnTheme::light(),
            MaccnAppearance::Dark => MaccnTheme::dark(),
        });
        cx.notify();
    }

    fn overview(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let entity = cx.entity().downgrade();
        div()
            .w(px(720.))
            .max_w_full()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("maccn"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.label_secondary)
                            .child(
                                "macOS-inspired controls, built for GPUI on top of gpui-base.",
                            ),
                    ),
            )
            .child(div().w_full().grid().grid_cols(3).gap_1().children(
                COMPONENTS.iter().enumerate().map(|(ix, name)| {
                    let entity = entity.clone();
                    MacButton::new(("overview-item", ix))
                        .size(MacControlSize::Small)
                        .child(*name)
                        .on_click(move |_, _, cx| {
                            _ = entity.update(cx, |this, cx| {
                                this.component = (*name).to_owned();
                                this.navigation_enabled = true;
                                cx.notify();
                            });
                        })
                }),
            ))
    }

    fn page_title(&self, title: &str) -> impl IntoElement {
        div()
            .w_full()
            .mb_3()
            .child(
                div()
                    .text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child(title.to_string()),
            )
    }
}

impl Render for Showcase {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let content = match self.component.as_str() {
            "badge" => self.badge().into_any_element(),
            "box" => self.box_example().into_any_element(),
            "button" => self.button().into_any_element(),
            "checkbox" => self.checkbox(cx).into_any_element(),
            "glass-panel" => self.glass_panel().into_any_element(),
            "help-button" => self.help_button().into_any_element(),
            "label" => self.label().into_any_element(),
            "pop-up-button" => self.pop_up_button(cx).into_any_element(),
            "progress" => self.progress().into_any_element(),
            "radio-group" => self.radio_group(cx).into_any_element(),
            "search-field" => self.search_field().into_any_element(),
            "secure-field" => self.secure_field().into_any_element(),
            "segmented-control" => self.segmented(cx).into_any_element(),
            "separator" => self.separator().into_any_element(),
            "slider" => self.slider(cx).into_any_element(),
            "spinner" => self.spinner().into_any_element(),
            "stepper" => self.stepper(cx).into_any_element(),
            "switch" => self.switch(cx).into_any_element(),
            "text-field" => self.text_field().into_any_element(),
            _ => self.overview(cx).into_any_element(),
        };

        let show_back = self.navigation_enabled && self.component != "overview";
        let entity = cx.entity().downgrade();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.window_bg)
            .text_color(theme.label)
            .text_xs()
            .font_family("Inter")
            .child(
                div()
                    .h_10()
                    .flex_none()
                    .px_3()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(theme.separator)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .when(show_back, |this| {
                                this.child(
                                    MacButton::new("back-to-overview")
                                        .size(MacControlSize::Small)
                                        .child("All components")
                                        .on_click({
                                            let entity = entity.clone();
                                            move |_, _, cx| {
                                                _ = entity.update(cx, |this, cx| {
                                                    this.component = "overview".to_owned();
                                                    cx.notify();
                                                });
                                            }
                                        }),
                                )
                            })
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("maccn"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                MacButton::new("appearance-light")
                                    .size(MacControlSize::Small)
                                    .child("Light")
                                    .on_click({
                                        let entity = entity.clone();
                                        move |_, _, cx| {
                                            _ = entity.update(cx, |this, cx| {
                                                this.set_appearance(MaccnAppearance::Light, cx);
                                            });
                                        }
                                    }),
                            )
                            .child(
                                MacButton::new("appearance-dark")
                                    .size(MacControlSize::Small)
                                    .child("Dark")
                                    .on_click({
                                        let entity = entity.clone();
                                        move |_, _, cx| {
                                            _ = entity.update(cx, |this, cx| {
                                                this.set_appearance(MaccnAppearance::Dark, cx);
                                            });
                                        }
                                    }),
                            ),
                    ),
            )
            .child(
                div()
                    .id("showcase-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll)
                    .child(
                        div()
                            .min_h_full()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .p_4()
                            .child(div().flex_none().child(content)),
                    ),
            )
    }
}

fn section_title(title: &str) -> impl IntoElement {
    div()
        .mb_1()
        .mt_3()
        .text_xs()
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .child(title.to_string())
}

pub fn run(app: Application, component: impl Into<String>) {
    let component = component.into();
    app.run(move |cx: &mut App| {
        maccn::init(cx);
        #[cfg(not(target_family = "wasm"))]
        {
            cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
        }
        let options = WindowOptions {
            #[cfg(not(target_family = "wasm"))]
            window_bounds: Some(WindowBounds::centered(size(px(900.), px(680.)), cx)),
            ..WindowOptions::default()
        };
        cx.open_window(options, move |window, cx| {
            cx.new(|cx| Showcase::new(component, window, cx))
        })
        .expect("failed to open maccn example window");
        cx.activate(true);
    });
}

#[cfg(target_family = "wasm")]
pub fn run_embedded(app: Application, component: impl Into<String>) -> gpui::ApplicationHandle {
    let component = component.into();
    app.run_embedded(move |cx: &mut App| {
        maccn::init(cx);
        cx.text_system()
            .add_fonts(vec![std::borrow::Cow::Borrowed(
                include_bytes!("./fonts/Inter-Regular.ttf").as_slice(),
            )])
            .expect("failed to load maccn example font");
        cx.open_window(WindowOptions::default(), move |window, cx| {
            cx.new(|cx| Showcase::new(component, window, cx))
        })
        .expect("failed to open maccn example window");
        cx.activate(true);
    })
}

#[cfg(not(target_family = "wasm"))]
pub fn run_native(component: &str) {
    run(gpui_platform::application(), component.to_owned());
}
