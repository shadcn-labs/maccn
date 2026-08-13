//! Per-component demos mirroring the macvue playground.

use super::Showcase;
use gpui::{Axis, Context, IntoElement, ParentElement as _, Styled as _, div, px};
use maccn::{
    ButtonVariant, GlassMaterial, LabelStyle, MacBadge, MacBox, MacButton, MacCheckbox,
    MacControlSize, MacGlassPanel, MacHelpButton, MacLabel, MacPopUpButton, MacPopUpButtonItem,
    MacProgress, MacRadio, MacRadioGroup, MacSearchField, MacSecureField, MacSegment,
    MacSegmentedControl, MacSeparator, MacSlider, MacSpinner, MacStepper, MacSwitch, MacTextField,
    ProgressSize, SpinnerSize,
};

fn section(title: &str) -> impl IntoElement {
    div()
        .mt_3()
        .mb_1()
        .text_xs()
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .child(title.to_string())
}

fn group(cx: &Context<Showcase>) -> impl IntoElement {
    let theme = maccn::MaccnTheme::global(cx);
    div().flex().flex_col().gap_2().p(px(12.)).rounded(px(12.)).bg(theme.group_box)
}

impl Showcase {
    pub(super) fn badge(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_3()
            .child(MacLabel::new("badge-label").child("Inbox"))
            .child(MacBadge::new("badge-count").child("128"))
    }

    pub(super) fn box_example(&self) -> impl IntoElement {
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Box"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        MacLabel::new("box-caption")
                            .style(LabelStyle::Footnote)
                            .secondary(true)
                            .child("General"),
                    )
                    .child(
                        MacBox::new("box")
                            .w_full()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(div().child("Automatically keep files up to date"))
                                    .child(
                                        MacSwitch::new("box-switch")
                                            .checked(self.switch_checked)
                                            .on_change(|_, _, _, cx| cx.refresh_windows()),
                                    ),
                            )
                            .child(MacSeparator::new("box-sep"))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(div().child("Show preview icons"))
                                    .child(
                                        MacSwitch::new("box-switch-2")
                                            .checked(true)
                                            .on_change(|_, _, _, cx| cx.refresh_windows()),
                                    ),
                            ),
                    ),
            )
    }

    pub(super) fn button(&self) -> impl IntoElement {
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Button"))
            .child(section("Variants"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(MacButton::new("btn-bordered").child("Bordered"))
                    .child(
                        MacButton::new("btn-prominent")
                            .variant(ButtonVariant::Prominent)
                            .child("Prominent"),
                    )
                    .child(
                        MacButton::new("btn-destructive")
                            .variant(ButtonVariant::Destructive)
                            .child("Delete"),
                    ),
            )
            .child(section("Sizes"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        MacButton::new("btn-xl")
                            .size(MacControlSize::ExtraLarge)
                            .variant(ButtonVariant::Prominent)
                            .child("Extra Large"),
                    )
                    .child(
                        MacButton::new("btn-l")
                            .size(MacControlSize::Large)
                            .child("Large"),
                    )
                    .child(MacButton::new("btn-r").child("Regular"))
                    .child(MacButton::new("btn-s").size(MacControlSize::Small).child("Small"))
                    .child(MacButton::new("btn-m").size(MacControlSize::Mini).child("Mini")),
            )
            .child(section("Disabled"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(MacButton::new("btn-d1").disabled(true).child("Bordered"))
                    .child(
                        MacButton::new("btn-d2")
                            .variant(ButtonVariant::Prominent)
                            .disabled(true)
                            .child("Prominent"),
                    )
                    .child(
                        MacButton::new("btn-d3")
                            .variant(ButtonVariant::Destructive)
                            .disabled(true)
                            .child("Delete"),
                    ),
            )
    }

    pub(super) fn checkbox(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let checked = self.checkbox_checked;
        let entity = cx.entity().downgrade();
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Checkbox"))
            .child(
                MacCheckbox::new("check-1")
                    .checked(checked)
                    .on_change(move |state, _, _, cx| {
                        let entity = entity.clone();
                        _ = entity.update(cx, |this, cx| {
                            this.checkbox_checked = state == maccn::MacCheckboxState::Checked;
                            cx.notify();
                        });
                    })
                    .child("Enable product updates"),
            )
            .child(MacCheckbox::new("check-2").checked(true).child("Checked"))
            .child(
                MacCheckbox::new("check-3")
                    .indeterminate(true)
                    .child("Mixed"),
            )
            .child(MacCheckbox::new("check-4").disabled(true).child("Disabled"))
            .child(section("Sizes"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        MacCheckbox::new("check-xl")
                            .size(MacControlSize::ExtraLarge)
                            .checked(true)
                            .child("XL"),
                    )
                    .child(
                        MacCheckbox::new("check-l")
                            .size(MacControlSize::Large)
                            .checked(true)
                            .child("L"),
                    )
                    .child(MacCheckbox::new("check-r").checked(true).child("R"))
                    .child(
                        MacCheckbox::new("check-s")
                            .size(MacControlSize::Small)
                            .checked(true)
                            .child("S"),
                    )
                    .child(
                        MacCheckbox::new("check-m")
                            .size(MacControlSize::Mini)
                            .checked(true)
                            .child("M"),
                    ),
            )
    }

    pub(super) fn glass_panel(&self) -> impl IntoElement {
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Glass Panel"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        MacGlassPanel::new("glass-1")
                            .material(GlassMaterial::Regular)
                            .w(px(180.))
                            .h(px(120.))
                            .child(div().text_sm().child("Regular")),
                    )
                    .child(
                        MacGlassPanel::new("glass-2")
                            .material(GlassMaterial::Clear)
                            .w(px(180.))
                            .h(px(120.))
                            .child(div().text_sm().child("Clear")),
                    ),
            )
    }

    pub(super) fn help_button(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Help Button"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        MacHelpButton::new("help-xl")
                            .size(MacControlSize::ExtraLarge)
                            .on_click(move |_, _, cx| cx.refresh_windows()),
                    )
                    .child(
                        MacHelpButton::new("help-l")
                            .size(MacControlSize::Large)
                            .on_click(move |_, _, cx| cx.refresh_windows()),
                    )
                    .child(MacHelpButton::new("help-r"))
                    .child(MacHelpButton::new("help-s").size(MacControlSize::Small))
                    .child(MacHelpButton::new("help-m").size(MacControlSize::Mini))
                    .child(MacHelpButton::new("help-d").disabled(true)),
            )
    }

    pub(super) fn label(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_start()
            .gap_1()
            .child(section("Label"))
            .child(MacLabel::new("l-1").style(LabelStyle::LargeTitle).child("Large Title"))
            .child(MacLabel::new("l-2").style(LabelStyle::Title1).child("Title 1"))
            .child(MacLabel::new("l-3").style(LabelStyle::Title2).child("Title 2"))
            .child(MacLabel::new("l-4").style(LabelStyle::Title3).child("Title 3"))
            .child(MacLabel::new("l-5").style(LabelStyle::Headline).child("Headline"))
            .child(MacLabel::new("l-6").style(LabelStyle::Body).child("Body"))
            .child(MacLabel::new("l-7").style(LabelStyle::Callout).child("Callout"))
            .child(MacLabel::new("l-8").style(LabelStyle::Subheadline).child("Subheadline"))
            .child(MacLabel::new("l-9").style(LabelStyle::Footnote).child("Footnote"))
            .child(MacLabel::new("l-10").style(LabelStyle::Caption1).child("Caption 1"))
            .child(
                MacLabel::new("l-11")
                    .style(LabelStyle::Body)
                    .secondary(true)
                    .child("Secondary body"),
            )
    }

    pub(super) fn pop_up_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().downgrade();
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Pop-Up Button"))
            .child(
                MacPopUpButton::new("popup")
                    .selected(self.popup_selected)
                    .placeholder("Select a color")
                    .items(vec![
                        MacPopUpButtonItem::new("Red").on_select({
                            let entity = entity.clone();
                            move |_, cx| {
                                _ = entity.update(cx, |this, cx| {
                                    this.popup_selected = Some(0);
                                    cx.notify();
                                });
                            }
                        }),
                        MacPopUpButtonItem::new("Green").on_select({
                            let entity = entity.clone();
                            move |_, cx| {
                                _ = entity.update(cx, |this, cx| {
                                    this.popup_selected = Some(1);
                                    cx.notify();
                                });
                            }
                        }),
                        MacPopUpButtonItem::new("Blue").on_select({
                            let entity = entity.clone();
                            move |_, cx| {
                                _ = entity.update(cx, |this, cx| {
                                    this.popup_selected = Some(2);
                                    cx.notify();
                                });
                            }
                        }),
                        MacPopUpButtonItem::separator(),
                        MacPopUpButtonItem::new("Custom…").disabled(true),
                    ]),
            )
            .child(section("Sizes"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        MacPopUpButton::new("popup-xl")
                            .size(MacControlSize::ExtraLarge)
                            .placeholder("Extra Large"),
                    )
                    .child(
                        MacPopUpButton::new("popup-l")
                            .size(MacControlSize::Large)
                            .placeholder("Large"),
                    )
                    .child(MacPopUpButton::new("popup-r").placeholder("Regular"))
                    .child(
                        MacPopUpButton::new("popup-s")
                            .size(MacControlSize::Small)
                            .placeholder("Small"),
                    )
                    .child(
                        MacPopUpButton::new("popup-m")
                            .size(MacControlSize::Mini)
                            .placeholder("Mini"),
                    ),
            )
            .child(MacPopUpButton::new("popup-d").disabled(true).placeholder("Disabled"))
    }

    pub(super) fn progress(&self) -> impl IntoElement {
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Progress"))
            .child(
                MacProgress::new("progress-1")
                    .w_full()
                    .value(self.progress_value * 100.),
            )
            .child(
                MacProgress::new("progress-2")
                    .size(ProgressSize::Small)
                    .w_full()
                    .value(self.progress_value * 100.),
            )
            .child(section("Indeterminate"))
            .child(MacProgress::new("progress-3").indeterminate(true).w_full())
    }

    pub(super) fn radio_group(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.radio_selected;
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Radio Group"))
            .child(
                MacRadioGroup::new("radios")
                    .child(
                        MacRadio::new("radio-0")
                            .checked(selected == 0)
                            .on_change({
                                let entity = cx.entity().downgrade();
                                move |_, _, _, cx| {
                                    _ = entity.update(cx, |this, cx| {
                                        this.radio_selected = 0;
                                        cx.notify();
                                    });
                                }
                            })
                            .child("Wi-Fi"),
                    )
                    .child(
                        MacRadio::new("radio-1")
                            .checked(selected == 1)
                            .on_change({
                                let entity = cx.entity().downgrade();
                                move |_, _, _, cx| {
                                    _ = entity.update(cx, |this, cx| {
                                        this.radio_selected = 1;
                                        cx.notify();
                                    });
                                }
                            })
                            .child("Bluetooth"),
                    )
                    .child(
                        MacRadio::new("radio-2")
                            .checked(selected == 2)
                            .on_change({
                                let entity = cx.entity().downgrade();
                                move |_, _, _, cx| {
                                    _ = entity.update(cx, |this, cx| {
                                        this.radio_selected = 2;
                                        cx.notify();
                                    });
                                }
                            })
                            .child("AirDrop"),
                    ),
            )
            .child(section("Disabled"))
            .child(
                MacRadioGroup::new("radios-disabled")
                    .child(MacRadio::new("radio-d1").disabled(true).child("Disabled"))
                    .child(
                        MacRadio::new("radio-d2")
                            .checked(true)
                            .disabled(true)
                            .child("Disabled selected"),
                    ),
            )
    }

    pub(super) fn search_field(&self) -> impl IntoElement {
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Search Field"))
            .child(MacSearchField::new("search", &self.search_input).w_full())
            .child(section("Sizes"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        MacSearchField::new("search-xl", &self.search_input)
                            .size(MacControlSize::ExtraLarge)
                            .w(px(240.)),
                    )
                    .child(
                        MacSearchField::new("search-s", &self.search_input)
                            .size(MacControlSize::Small)
                            .w(px(180.)),
                    ),
            )
            .child(section("Disabled"))
            .child(
                MacSearchField::new("search-d", &self.search_input)
                    .disabled(true)
                    .w_full(),
            )
    }

    pub(super) fn secure_field(&self) -> impl IntoElement {
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Secure Field"))
            .child(MacSecureField::new("secure", &self.secure_input).w_full())
            .child(
                MacSecureField::new("secure-d", &self.secure_input)
                    .disabled(true)
                    .w_full(),
            )
    }

    pub(super) fn segmented(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let index = self.segmented_index;
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Segmented Control"))
            .child(
                MacSegmentedControl::new("segmented")
                    .w(px(280.))
                    .child(
                        MacSegment::new("seg-0")
                            .selected(index == 0)
                            .child("Day")
                            .on_click({
                                let entity = cx.entity().downgrade();
                                move |_, _, cx| {
                                    _ = entity.update(cx, |this, cx| {
                                        this.segmented_index = 0;
                                        cx.notify();
                                    });
                                }
                            }),
                    )
                    .child(
                        MacSegment::new("seg-1")
                            .selected(index == 1)
                            .child("Week")
                            .on_click({
                                let entity = cx.entity().downgrade();
                                move |_, _, cx| {
                                    _ = entity.update(cx, |this, cx| {
                                        this.segmented_index = 1;
                                        cx.notify();
                                    });
                                }
                            }),
                    )
                    .child(
                        MacSegment::new("seg-2")
                            .selected(index == 2)
                            .child("Month")
                            .on_click({
                                let entity = cx.entity().downgrade();
                                move |_, _, cx| {
                                    _ = entity.update(cx, |this, cx| {
                                        this.segmented_index = 2;
                                        cx.notify();
                                    });
                                }
                            }),
                    )
                    .child(
                        MacSegment::new("seg-3")
                            .selected(index == 3)
                            .child("Year")
                            .on_click({
                                let entity = cx.entity().downgrade();
                                move |_, _, cx| {
                                    _ = entity.update(cx, |this, cx| {
                                        this.segmented_index = 3;
                                        cx.notify();
                                    });
                                }
                            }),
                    ),
            )
            .child(section("Sizes"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        MacSegmentedControl::new("segmented-xl")
                            .size(MacControlSize::ExtraLarge)
                            .w(px(320.))
                            .child(MacSegment::new("seg-xl-0").selected(true).child("Left"))
                            .child(MacSegment::new("seg-xl-1").child("Right")),
                    )
                    .child(
                        MacSegmentedControl::new("segmented-m")
                            .size(MacControlSize::Mini)
                            .w(px(140.))
                            .child(MacSegment::new("seg-m-0").selected(true).child("On"))
                            .child(MacSegment::new("seg-m-1").child("Off")),
                    ),
            )
    }

    pub(super) fn separator(&self) -> impl IntoElement {
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Separator"))
            .child(MacSeparator::new("sep-1").w_full())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().child("Left"))
                    .child(MacSeparator::new("sep-2").axis(Axis::Vertical).h(px(16.)))
                    .child(div().child("Right")),
            )
    }

    pub(super) fn slider(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = maccn::MaccnTheme::global(cx);
        let value = self.slider.read(cx).value().start();
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Slider"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .child(div().child("Volume"))
                    .child(div().text_color(theme.label_secondary).child(format!("{value:.0}%"))),
            )
            .child(MacSlider::new("slider", &self.slider).w_full())
            .child(
                MacSlider::new("slider-disabled", &self.slider)
                    .disabled(true)
                    .w_full(),
            )
    }

    pub(super) fn spinner(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Spinner"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(MacSpinner::new("spinner-1"))
                    .child(
                        MacSpinner::new("spinner-2")
                            .size(SpinnerSize::Small),
                    ),
            )
    }

    pub(super) fn stepper(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let value = self.stepper_value;
        let entity = cx.entity().downgrade();
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Stepper"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(MacStepper::new("stepper").on_increment({
                        let entity = entity.clone();
                        move |_, cx| {
                            _ = entity.update(cx, |this, cx| {
                                this.stepper_value = (this.stepper_value + 1).min(10);
                                cx.notify();
                            });
                        }
                    }).on_decrement({
                        let entity = entity.clone();
                        move |_, cx| {
                            _ = entity.update(cx, |this, cx| {
                                this.stepper_value = (this.stepper_value - 1).max(0);
                                cx.notify();
                            });
                        }
                    }))
                    .child(div().child(value.to_string())),
            )
            .child(MacStepper::new("stepper-d").disabled(true))
    }

    pub(super) fn switch(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let checked = self.switch_checked;
        let entity = cx.entity().downgrade();
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Switch"))
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(div().child("Wi-Fi"))
                    .child(
                        MacSwitch::new("switch-1")
                            .checked(checked)
                            .on_change(move |next, _, _, cx| {
                                let entity = entity.clone();
                                _ = entity.update(cx, |this, cx| {
                                    this.switch_checked = next;
                                    cx.notify();
                                });
                            }),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(div().child("Automatic updates"))
                    .child(MacSwitch::new("switch-2").checked(true).on_change(|_, _, _, cx| cx.refresh_windows())),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(div().child("Disabled"))
                    .child(MacSwitch::new("switch-3").disabled(true)),
            )
            .child(section("Sizes"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(MacSwitch::new("switch-xl").size(MacControlSize::ExtraLarge).checked(true))
                    .child(MacSwitch::new("switch-l").size(MacControlSize::Large).checked(true))
                    .child(MacSwitch::new("switch-r").checked(true))
                    .child(MacSwitch::new("switch-s").size(MacControlSize::Small).checked(true))
                    .child(MacSwitch::new("switch-m").size(MacControlSize::Mini).checked(true)),
            )
    }

    pub(super) fn text_field(&self) -> impl IntoElement {
        div()
            .w_80()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(section("Text Field"))
            .child(MacTextField::new("field", &self.input).w_full())
            .child(section("Sizes"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        MacTextField::new("field-xl", &self.input)
                            .size(MacControlSize::ExtraLarge)
                            .w(px(220.)),
                    )
                    .child(
                        MacTextField::new("field-l", &self.input)
                            .size(MacControlSize::Large)
                            .w(px(180.)),
                    )
                    .child(MacTextField::new("field-r", &self.input).w(px(160.)))
                    .child(
                        MacTextField::new("field-s", &self.input)
                            .size(MacControlSize::Small)
                            .w(px(140.)),
                    )
                    .child(
                        MacTextField::new("field-m", &self.input)
                            .size(MacControlSize::Mini)
                            .w(px(120.)),
                    ),
            )
            .child(section("Disabled"))
            .child(MacTextField::new("field-d", &self.input).disabled(true).w_full())
    }
}
