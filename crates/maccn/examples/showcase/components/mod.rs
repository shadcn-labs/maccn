//! Per-component demos mirroring the macvue playground.

use super::Showcase;
use gpui::{AnyElement, Axis, Context, IntoElement, ParentElement as _, Styled as _, div, px, relative};
use maccn::{
    ButtonVariant, GlassMaterial, LabelStyle, MacBadge, MacBox, MacButton, MacCheckbox,
    MacCheckboxState, MacControlSize, MacGlassPanel, MacHelpButton, MacLabel, MacPopUpButton,
    MacPopUpButtonItem, MacProgress, MacRadio, MacRadioGroup, MacSearchField, MacSecureField,
    MacSegment, MacSegmentedControl, MacSeparator, MacSlider, MacSpinner, MacStepper, MacSwitch,
    MacTextField, ProgressSize, SpinnerSize,
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

    pub(super) fn box_example(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().downgrade();
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
                            .child("Network"),
                    )
                    .child(
                        MacBox::new("box")
                            .w(px(260.))
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                MacSwitch::new("box-wifi")
                                    .w_full()
                                    .justify_between()
                                    .checked(self.switch_checked)
                                    .on_change({
                                        let entity = entity.clone();
                                        move |next, _, _, cx| {
                                            _ = entity.update(cx, |this, cx| {
                                                this.switch_checked = next;
                                                cx.notify();
                                            });
                                        }
                                    })
                                    .child("Wi-Fi"),
                            )
                            .child(MacSeparator::new("box-sep"))
                            .child(
                                MacSwitch::new("box-bluetooth")
                                    .w_full()
                                    .justify_between()
                                    .checked(self.box_bluetooth_checked)
                                    .on_change({
                                        let entity = entity.clone();
                                        move |next, _, _, cx| {
                                            _ = entity.update(cx, |this, cx| {
                                                this.box_bluetooth_checked = next;
                                                cx.notify();
                                            });
                                        }
                                    })
                                    .child("Bluetooth"),
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
            .child(MacSlider::new("slider", &self.slider).w(px(220.)))
            .child(
                MacSlider::new("slider-disabled", &self.slider)
                    .disabled(true)
                    .w(px(220.)),
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

    pub(super) fn card_content(&self, cx: &mut Context<Self>) -> AnyElement {
        match self.component.as_str() {
            "button" => div()
                .flex()
                .items_center()
                .gap_2()
                .child(MacButton::new("card-btn").child("Default"))
                .child(
                    MacButton::new("card-btn-p")
                        .variant(ButtonVariant::Prominent)
                        .child("Prominent"),
                )
                .into_any_element(),
            "checkbox" => MacCheckbox::new("card-chk")
                .checked(self.checkbox_checked)
                .on_change({
                    let entity = cx.entity().downgrade();
                    move |state, _, _, cx| {
                        _ = entity.update(cx, |this, cx| {
                            this.checkbox_checked = state == maccn::MacCheckboxState::Checked;
                            cx.notify();
                        });
                    }
                })
                .child("Show all filename extensions")
                .into_any_element(),
            "radio-group" => MacRadioGroup::new("card-radios")
                .child(
                    MacRadio::new("card-radio-0")
                        .checked(self.radio_selected == 0)
                        .on_change({
                            let entity = cx.entity().downgrade();
                            move |_, _, _, cx| {
                                _ = entity.update(cx, |this, cx| {
                                    this.radio_selected = 0;
                                    cx.notify();
                                });
                            }
                        })
                        .child("Name"),
                )
                .child(
                    MacRadio::new("card-radio-1")
                        .checked(self.radio_selected == 1)
                        .on_change({
                            let entity = cx.entity().downgrade();
                            move |_, _, _, cx| {
                                _ = entity.update(cx, |this, cx| {
                                    this.radio_selected = 1;
                                    cx.notify();
                                });
                            }
                        })
                        .child("Kind"),
                )
                .child(
                    MacRadio::new("card-radio-2")
                        .checked(self.radio_selected == 2)
                        .on_change({
                            let entity = cx.entity().downgrade();
                            move |_, _, _, cx| {
                                _ = entity.update(cx, |this, cx| {
                                    this.radio_selected = 2;
                                    cx.notify();
                                });
                            }
                        })
                        .child("Date modified"),
                )
                .into_any_element(),
            "switch" => div()
                .flex()
                .items_center()
                .justify_between()
                .w(px(180.))
                .child(div().child("Wi-Fi"))
                .child(
                    MacSwitch::new("card-switch")
                        .checked(self.switch_checked)
                        .on_change({
                            let entity = cx.entity().downgrade();
                            move |next, _, _, cx| {
                                let entity = entity.clone();
                                _ = entity.update(cx, |this, cx| {
                                    this.switch_checked = next;
                                    cx.notify();
                                });
                            }
                        }),
                )
                .into_any_element(),
            "segmented-control" => MacSegmentedControl::new("card-seg")
                .size(MacControlSize::Small)
                .w(px(140.))
                .child(
                    MacSegment::new("card-seg-0")
                        .selected(self.segmented_index == 0)
                        .child("List")
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
                    MacSegment::new("card-seg-1")
                        .selected(self.segmented_index == 1)
                        .child("Icons")
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
                .into_any_element(),
            "pop-up-button" => MacPopUpButton::new("card-popup")
                .selected(self.popup_selected)
                .placeholder("Red")
                .items(vec![
                    MacPopUpButtonItem::new("Red").on_select({
                        let entity = cx.entity().downgrade();
                        move |_, cx| {
                            _ = entity.update(cx, |this, cx| {
                                this.popup_selected = Some(0);
                                cx.notify();
                            });
                        }
                    }),
                    MacPopUpButtonItem::new("Green").on_select({
                        let entity = cx.entity().downgrade();
                        move |_, cx| {
                            _ = entity.update(cx, |this, cx| {
                                this.popup_selected = Some(1);
                                cx.notify();
                            });
                        }
                    }),
                    MacPopUpButtonItem::new("Blue").on_select({
                        let entity = cx.entity().downgrade();
                        move |_, cx| {
                            _ = entity.update(cx, |this, cx| {
                                this.popup_selected = Some(2);
                                cx.notify();
                            });
                        }
                    }),
                ])
                .into_any_element(),
            "progress" => MacProgress::new("card-prog")
                .w(px(160.))
                .value(65.)
                .into_any_element(),
            "spinner" => MacSpinner::new("card-spinner").into_any_element(),
            "slider" => MacSlider::new("card-slider", &self.slider)
                .w(px(180.))
                .into_any_element(),
            "stepper" => MacStepper::new("card-stepper")
                .on_increment({
                    let entity = cx.entity().downgrade();
                    move |_, cx| {
                        _ = entity.update(cx, |this, cx| {
                            this.stepper_value = (this.stepper_value + 1).min(99);
                            cx.notify();
                        });
                    }
                })
                .on_decrement({
                    let entity = cx.entity().downgrade();
                    move |_, cx| {
                        _ = entity.update(cx, |this, cx| {
                            this.stepper_value = (this.stepper_value - 1).max(1);
                            cx.notify();
                        });
                    }
                })
                .into_any_element(),
            "label" => div()
                .flex()
                .flex_col()
                .gap_1()
                .child(MacLabel::new("card-lbl-1").style(LabelStyle::Title3).child("Storage"))
                .child(
                    MacLabel::new("card-lbl-2")
                        .style(LabelStyle::Footnote)
                        .secondary(true)
                        .child("Manage space on this Mac."),
                )
                .into_any_element(),
            "badge" => div()
                .flex()
                .items_center()
                .justify_between()
                .w(px(140.))
                .child(MacLabel::new("badge-label").child("Inbox"))
                .child(MacBadge::new("badge-count").child("128"))
                .into_any_element(),
            "box" => MacBox::new("card-box")
                .w(px(160.))
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    MacLabel::new("card-box-lbl")
                        .style(LabelStyle::Footnote)
                        .secondary(true)
                        .child("Network"),
                )
                .child(
                    MacSwitch::new("card-box-wifi")
                        .w_full()
                        .justify_between()
                        .checked(true)
                        .on_change(|_, _, _, cx| cx.refresh_windows())
                        .child("Wi-Fi"),
                )
                .child(
                    MacSwitch::new("card-box-bluetooth")
                        .w_full()
                        .justify_between()
                        .checked(false)
                        .on_change(|_, _, _, cx| cx.refresh_windows())
                        .child("Bluetooth"),
                )
                .into_any_element(),
            "separator" => div()
                .flex()
                .flex_col()
                .gap_2()
                .w(px(140.))
                .child(MacLabel::new("card-sep-above").child("Displays"))
                .child(MacSeparator::new("card-sep"))
                .child(MacLabel::new("card-sep-below").child("Wallpaper"))
                .into_any_element(),
            "glass-panel" => {
                let theme = maccn::MaccnTheme::global(cx);
                div()
                    .relative()
                    .w(px(180.))
                    .h(px(120.))
                    .child(maccn::stage_grid_bg(theme.label_quaternary).w_full().h_full())
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                MacGlassPanel::new("card-glass")
                                    .material(GlassMaterial::Clear)
                                    .w(px(120.))
                                    .h(px(64.))
                                    .child(div().text_sm().child("Media controls")),
                            ),
                    )
                    .into_any_element()
            }
            "help-button" => MacHelpButton::new("card-help").into_any_element(),
            "text-field" => MacTextField::new("card-field", &self.input)
                .w(px(180.))
                .into_any_element(),
            "secure-field" => MacSecureField::new("card-secure", &self.secure_input)
                .w(px(180.))
                .into_any_element(),
            "search-field" => MacSearchField::new("card-search", &self.search_input)
                .w(px(180.))
                .into_any_element(),
            _ => self.button().into_any_element(),
        }
    }

    // ------------------------------------------------------------------
    // Variant previews for the WASM docs.
    // ------------------------------------------------------------------

    fn button_basic(&self) -> impl IntoElement {
        MacButton::new("btn-basic").child("Click me")
    }

    fn button_variants(&self) -> impl IntoElement {
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
            )
    }

    fn button_sizes(&self) -> impl IntoElement {
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
            .child(MacButton::new("btn-m").size(MacControlSize::Mini).child("Mini"))
    }

    fn button_disabled(&self) -> impl IntoElement {
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
            )
    }

    fn checkbox_basic(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let checked = self.checkbox_checked;
        let entity = cx.entity().downgrade();
        MacCheckbox::new("check-basic")
            .checked(checked)
            .on_change(move |state, _, _, cx| {
                _ = entity.update(cx, |this, cx| {
                    this.checkbox_checked = state == MacCheckboxState::Checked;
                    cx.notify();
                });
            })
            .child("Enable product updates")
    }

    fn checkbox_states(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(MacCheckbox::new("check-unchecked").child("Unchecked"))
            .child(MacCheckbox::new("check-checked").checked(true).child("Checked"))
            .child(
                MacCheckbox::new("check-mixed")
                    .indeterminate(true)
                    .child("Indeterminate"),
            )
    }

    fn checkbox_sizes(&self) -> impl IntoElement {
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
            )
    }

    fn checkbox_disabled(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(MacCheckbox::new("check-d1").disabled(true).child("Disabled"))
            .child(
                MacCheckbox::new("check-d2")
                    .checked(true)
                    .disabled(true)
                    .child("Disabled checked"),
            )
    }

    fn help_button_basic(&self) -> impl IntoElement {
        MacHelpButton::new("help-basic")
            .on_click(move |_, _, cx| cx.refresh_windows())
    }

    fn help_button_sizes(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(MacHelpButton::new("help-xl").size(MacControlSize::ExtraLarge))
            .child(MacHelpButton::new("help-l").size(MacControlSize::Large))
            .child(MacHelpButton::new("help-r"))
            .child(MacHelpButton::new("help-s").size(MacControlSize::Small))
            .child(MacHelpButton::new("help-m").size(MacControlSize::Mini))
    }

    fn help_button_disabled(&self) -> impl IntoElement {
        MacHelpButton::new("help-d").disabled(true)
    }

    fn label_basic(&self) -> impl IntoElement {
        MacLabel::new("label-basic").child("Notifications")
    }

    fn label_variants(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(MacLabel::new("l-lt").style(LabelStyle::LargeTitle).child("Large Title"))
            .child(MacLabel::new("l-t1").style(LabelStyle::Title1).child("Title 1"))
            .child(MacLabel::new("l-t2").style(LabelStyle::Title2).child("Title 2"))
            .child(MacLabel::new("l-t3").style(LabelStyle::Title3).child("Title 3"))
            .child(MacLabel::new("l-hl").style(LabelStyle::Headline).child("Headline"))
            .child(MacLabel::new("l-body").style(LabelStyle::Body).child("Body"))
            .child(MacLabel::new("l-co").style(LabelStyle::Callout).child("Callout"))
            .child(MacLabel::new("l-sh").style(LabelStyle::Subheadline).child("Subheadline"))
            .child(MacLabel::new("l-fn").style(LabelStyle::Footnote).child("Footnote"))
            .child(MacLabel::new("l-c1").style(LabelStyle::Caption1).child("Caption 1"))
            .child(MacLabel::new("l-c2").style(LabelStyle::Caption2).child("Caption 2"))
    }

    fn pop_up_button_basic(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().downgrade();
        MacPopUpButton::new("popup-basic")
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
            ])
    }

    fn pop_up_button_sizes(&self) -> impl IntoElement {
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
            )
    }

    fn pop_up_button_states(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                MacPopUpButton::new("popup-d")
                    .disabled(true)
                    .placeholder("Disabled"),
            )
            .child(
                MacPopUpButton::new("popup-item-d")
                    .placeholder("With disabled item")
                    .items(vec![
                        MacPopUpButtonItem::new("Red"),
                        MacPopUpButtonItem::new("Green").disabled(true),
                        MacPopUpButtonItem::new("Blue"),
                    ]),
            )
    }

    fn progress_basic(&self) -> impl IntoElement {
        MacProgress::new("progress-basic")
            .w(px(220.))
            .value(self.progress_value * 100.)
    }

    fn progress_indeterminate(&self) -> impl IntoElement {
        MacProgress::new("progress-indeterminate")
            .w(px(220.))
            .indeterminate(true)
    }

    fn progress_sizes(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .w(px(220.))
            .child(
                MacProgress::new("progress-r")
                    .w_full()
                    .value(self.progress_value * 100.),
            )
            .child(
                MacProgress::new("progress-s")
                    .size(ProgressSize::Small)
                    .w_full()
                    .value(self.progress_value * 100.),
            )
    }

    fn radio_group_basic(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.radio_selected;
        MacRadioGroup::new("radios-basic")
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
            )
    }

    fn radio_group_horizontal(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.radio_selected;
        MacRadioGroup::new("radios-horizontal")
            .axis(Axis::Horizontal)
            .child(
                MacRadio::new("radio-h-0")
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
                    .child("Yes"),
            )
            .child(
                MacRadio::new("radio-h-1")
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
                    .child("No"),
            )
    }

    fn radio_group_sizes(&self) -> impl IntoElement {
        MacRadioGroup::new("radios-sizes")
            .size(MacControlSize::Small)
            .child(MacRadio::new("radio-s").checked(true).child("Small"))
            .child(MacRadio::new("radio-s2").child("Choice"))
    }

    fn radio_group_disabled(&self) -> impl IntoElement {
        MacRadioGroup::new("radios-disabled")
            .child(MacRadio::new("radio-d1").disabled(true).child("Disabled"))
            .child(
                MacRadio::new("radio-d2")
                    .checked(true)
                    .disabled(true)
                    .child("Disabled selected"),
            )
    }

    fn search_field_basic(&self) -> impl IntoElement {
        MacSearchField::new("search-basic", &self.search_input).w(px(220.))
    }

    fn search_field_clearing(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(MacSearchField::new("search-clear", &self.search_input).w(px(220.)))
            .child(
                MacLabel::new("search-hint")
                    .style(LabelStyle::Footnote)
                    .secondary(true)
                    .child("Type text to reveal the clear button; press Escape to clear."),
            )
    }

    fn search_field_sizes(&self) -> impl IntoElement {
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
            )
    }

    fn search_field_disabled(&self) -> impl IntoElement {
        MacSearchField::new("search-d", &self.search_input)
            .disabled(true)
            .w(px(220.))
    }

    fn secure_field_basic(&self) -> impl IntoElement {
        MacSecureField::new("secure-basic", &self.secure_input).w(px(220.))
    }

    fn secure_field_sizes(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                MacSecureField::new("secure-xl", &self.secure_input)
                    .size(MacControlSize::ExtraLarge)
                    .w(px(220.)),
            )
            .child(
                MacSecureField::new("secure-s", &self.secure_input)
                    .size(MacControlSize::Small)
                    .w(px(160.)),
            )
    }

    fn secure_field_disabled(&self) -> impl IntoElement {
        MacSecureField::new("secure-d", &self.secure_input)
            .disabled(true)
            .w(px(220.))
    }

    fn segmented_basic(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let index = self.segmented_index;
        MacSegmentedControl::new("segmented-basic")
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
    }

    fn segmented_multiple(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = &self.segmented_multiple;
        MacSegmentedControl::new("segmented-multiple")
            .w(px(220.))
            .child(self.multiple_segment("seg-m-0", 0, "B", selected, cx))
            .child(self.multiple_segment("seg-m-1", 1, "I", selected, cx))
            .child(self.multiple_segment("seg-m-2", 2, "U", selected, cx))
    }

    fn multiple_segment(
        &self,
        id: impl Into<gpui::ElementId>,
        index: usize,
        label: &str,
        selected: &[usize],
        cx: &mut Context<Self>,
    ) -> MacSegment {
        let is_selected = selected.contains(&index);
        let entity = cx.entity().downgrade();
        MacSegment::new(id)
            .selected(is_selected)
            .child(label.to_string())
            .on_click({
                let label = label.to_string();
                move |_, _, cx| {
                    _ = entity.update(cx, |this, cx| {
                        if let Some(pos) = this.segmented_multiple.iter().position(|i| *i == index) {
                            this.segmented_multiple.remove(pos);
                        } else {
                            this.segmented_multiple.push(index);
                        }
                        this.segmented_multiple.sort();
                        cx.notify();
                    });
                    let _ = label;
                }
            })
    }

    fn segmented_sizes(&self) -> impl IntoElement {
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
            )
    }

    fn segmented_disabled(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                MacSegmentedControl::new("segmented-d")
                    .w(px(220.))
                    .disabled(true)
                    .child(MacSegment::new("seg-d-0").selected(true).child("On"))
                    .child(MacSegment::new("seg-d-1").child("Off")),
            )
    }

    fn separator_basic(&self) -> impl IntoElement {
        MacSeparator::new("sep-basic").w(px(220.))
    }

    fn separator_vertical(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .h(px(32.))
            .child(div().child("Left"))
            .child(MacSeparator::new("sep-v").axis(Axis::Vertical))
            .child(div().child("Right"))
    }

    fn slider_basic(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = maccn::MaccnTheme::global(cx);
        let value = self.slider.read(cx).value().start();
        div()
            .w(px(260.))
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .child(div().child("Volume"))
                    .child(div().text_color(theme.label_secondary).child(format!("{value:.0}%"))),
            )
            .child(MacSlider::new("slider-basic", &self.slider).w_full())
    }

    fn slider_ticks(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = maccn::MaccnTheme::global(cx);
        let ticks = vec![0., 25., 50., 75., 100.];
        div()
            .w(px(260.))
            .flex()
            .flex_col()
            .gap_1()
            .child(MacSlider::new("slider-ticks", &self.slider).w_full())
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(12.))
                    .children(ticks.into_iter().map(|t| {
                        div()
                            .absolute()
                            .left(relative(t / 100.))
                            .ml(px(-1.))
                            .w(px(2.))
                            .h(px(6.))
                            .rounded_full()
                            .bg(theme.label_quaternary)
                            .into_any_element()
                    })),
            )
    }

    fn slider_vertical(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = maccn::MaccnTheme::global(cx);
        let value = self.slider.read(cx).value().start();
        let percentage = value / 100.;
        div()
            .h(px(160.))
            .flex()
            .items_center()
            .gap_3()
            .child(
                div()
                    .relative()
                    .w(px(6.))
                    .h_full()
                    .rounded_full()
                    .bg(theme.control_track)
                    .child(
                        div()
                            .absolute()
                            .bottom_0()
                            .left_0()
                            .right_0()
                            .h(relative(percentage.clamp(0., 1.)))
                            .rounded_full()
                            .bg(theme.accent),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(px(-5.))
                            .bottom(relative(percentage.clamp(0., 1.)))
                            .mb(px(-8.))
                            .size(px(16.))
                            .rounded_full()
                            .bg(theme.slider_knob)
                            .shadow(maccn::switch_thumb_shadow()),
                    ),
            )
            .child(
                div()
                    .text_color(theme.label_secondary)
                    .child(format!("{value:.0}%")),
            )
    }

    fn slider_sizes(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(260.))
            .flex()
            .flex_col()
            .gap_3()
            .child(MacSlider::new("slider-xl", &self.slider).size(MacControlSize::ExtraLarge))
            .child(MacSlider::new("slider-s", &self.slider).size(MacControlSize::Small))
    }

    fn slider_disabled(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        MacSlider::new("slider-disabled", &self.slider)
            .disabled(true)
            .w(px(260.))
    }

    fn spinner_basic(&self) -> impl IntoElement {
        MacSpinner::new("spinner-basic")
    }

    fn spinner_sizes(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(MacSpinner::new("spinner-r"))
            .child(MacSpinner::new("spinner-s").size(SpinnerSize::Small))
    }

    fn stepper_basic(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let value = self.stepper_value;
        let entity = cx.entity().downgrade();
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                MacStepper::new("stepper-basic").on_increment({
                    let entity = entity.clone();
                    move |_, cx| {
                        _ = entity.update(cx, |this, cx| {
                            this.stepper_value = (this.stepper_value + 1).min(99);
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
                }),
            )
            .child(div().child(value.to_string()))
    }

    fn stepper_with_text_field(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().downgrade();
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(MacTextField::new("stepper-field", &self.stepper_input).w(px(60.)))
            .child(
                MacStepper::new("stepper-paired").on_increment({
                    let entity = entity.clone();
                    move |window, cx| {
                        _ = entity.update(cx, |this, cx| {
                            this.stepper_value = (this.stepper_value + 1).min(99);
                            let value = this.stepper_value;
                            this.stepper_input.update(cx, |state, cx| {
                                state.set_value(value.to_string(), window, cx);
                            });
                            cx.notify();
                        });
                    }
                }).on_decrement({
                    let entity = entity.clone();
                    move |window, cx| {
                        _ = entity.update(cx, |this, cx| {
                            this.stepper_value = (this.stepper_value - 1).max(0);
                            let value = this.stepper_value;
                            this.stepper_input.update(cx, |state, cx| {
                                state.set_value(value.to_string(), window, cx);
                            });
                            cx.notify();
                        });
                    }
                }),
            )
    }

    fn stepper_sizes(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(MacStepper::new("stepper-xl").size(MacControlSize::ExtraLarge))
            .child(MacStepper::new("stepper-l").size(MacControlSize::Large))
            .child(MacStepper::new("stepper-r"))
            .child(MacStepper::new("stepper-s").size(MacControlSize::Small))
            .child(MacStepper::new("stepper-m").size(MacControlSize::Mini))
    }

    fn stepper_disabled(&self) -> impl IntoElement {
        MacStepper::new("stepper-d").disabled(true)
    }

    fn switch_basic(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let checked = self.switch_checked;
        let entity = cx.entity().downgrade();
        div()
            .w(px(220.))
            .flex()
            .items_center()
            .justify_between()
            .child(div().child("Wi-Fi"))
            .child(
                MacSwitch::new("switch-basic")
                    .checked(checked)
                    .on_change(move |next, _, _, cx| {
                        _ = entity.update(cx, |this, cx| {
                            this.switch_checked = next;
                            cx.notify();
                        });
                    }),
            )
    }

    fn switch_sizes(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(MacSwitch::new("switch-xl").size(MacControlSize::ExtraLarge).checked(true))
            .child(MacSwitch::new("switch-l").size(MacControlSize::Large).checked(true))
            .child(MacSwitch::new("switch-r").checked(true))
            .child(MacSwitch::new("switch-s").size(MacControlSize::Small).checked(true))
            .child(MacSwitch::new("switch-m").size(MacControlSize::Mini).checked(true))
    }

    fn switch_disabled(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(MacSwitch::new("switch-d1").disabled(true).child("Disabled"))
            .child(
                MacSwitch::new("switch-d2")
                    .checked(true)
                    .disabled(true)
                    .child("Disabled on"),
            )
    }

    fn text_field_basic(&self) -> impl IntoElement {
        MacTextField::new("field-basic", &self.input).w(px(220.))
    }

    fn text_field_labelled(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(MacLabel::new("field-label").child("Server:"))
            .child(MacTextField::new("field-labelled", &self.input).w(px(200.)))
    }

    fn text_field_placeholder(&self) -> impl IntoElement {
        MacTextField::new("field-placeholder", &self.input).w(px(220.))
    }

    fn text_field_sizes(&self) -> impl IntoElement {
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
            )
    }

    fn text_field_disabled(&self) -> impl IntoElement {
        MacTextField::new("field-d", &self.input)
            .disabled(true)
            .w(px(220.))
    }

    pub(super) fn variant(
        &self,
        component: &str,
        variant: &str,
        cx: &mut Context<Self>,
    ) -> gpui::AnyElement {
        match (component, variant) {
            ("badge", "Basic") => self.badge().into_any_element(),
            ("box", "Basic") => self.box_example(cx).into_any_element(),
            ("button", "Basic") => self.button_basic().into_any_element(),
            ("button", "Variants") => self.button_variants().into_any_element(),
            ("button", "Sizes") => self.button_sizes().into_any_element(),
            ("button", "Disabled") => self.button_disabled().into_any_element(),
            ("checkbox", "Basic") => self.checkbox_basic(cx).into_any_element(),
            ("checkbox", "States") => self.checkbox_states().into_any_element(),
            ("checkbox", "Sizes") => self.checkbox_sizes().into_any_element(),
            ("checkbox", "Disabled") => self.checkbox_disabled().into_any_element(),
            ("glass-panel", "Basic") => self.glass_panel().into_any_element(),
            ("help-button", "Basic") => self.help_button_basic().into_any_element(),
            ("help-button", "Sizes") => self.help_button_sizes().into_any_element(),
            ("help-button", "Disabled") => self.help_button_disabled().into_any_element(),
            ("label", "Basic") => self.label_basic().into_any_element(),
            ("label", "Variants") => self.label_variants().into_any_element(),
            ("pop-up-button", "Basic") => self.pop_up_button_basic(cx).into_any_element(),
            ("pop-up-button", "Sizes") => self.pop_up_button_sizes().into_any_element(),
            ("pop-up-button", "States") => self.pop_up_button_states().into_any_element(),
            ("progress", "Basic") => self.progress_basic().into_any_element(),
            ("progress", "Indeterminate") => self.progress_indeterminate().into_any_element(),
            ("progress", "Sizes") => self.progress_sizes().into_any_element(),
            ("radio-group", "Basic") => self.radio_group_basic(cx).into_any_element(),
            ("radio-group", "Horizontal") => self.radio_group_horizontal(cx).into_any_element(),
            ("radio-group", "Sizes") => self.radio_group_sizes().into_any_element(),
            ("radio-group", "Disabled") => self.radio_group_disabled().into_any_element(),
            ("search-field", "Basic") => self.search_field_basic().into_any_element(),
            ("search-field", "Clearing") => self.search_field_clearing().into_any_element(),
            ("search-field", "Sizes") => self.search_field_sizes().into_any_element(),
            ("search-field", "Disabled") => self.search_field_disabled().into_any_element(),
            ("secure-field", "Basic") => self.secure_field_basic().into_any_element(),
            ("secure-field", "Sizes") => self.secure_field_sizes().into_any_element(),
            ("secure-field", "Disabled") => self.secure_field_disabled().into_any_element(),
            ("segmented-control", "Basic") => self.segmented_basic(cx).into_any_element(),
            ("segmented-control", "Multiple") => self.segmented_multiple(cx).into_any_element(),
            ("segmented-control", "Sizes") => self.segmented_sizes().into_any_element(),
            ("segmented-control", "Disabled") => self.segmented_disabled().into_any_element(),
            ("separator", "Basic") => self.separator_basic().into_any_element(),
            ("separator", "Vertical") => self.separator_vertical().into_any_element(),
            ("slider", "Basic") => self.slider_basic(cx).into_any_element(),
            ("slider", "Ticks") => self.slider_ticks(cx).into_any_element(),
            ("slider", "Vertical") => self.slider_vertical(cx).into_any_element(),
            ("slider", "Sizes") => self.slider_sizes(cx).into_any_element(),
            ("slider", "Disabled") => self.slider_disabled(cx).into_any_element(),
            ("spinner", "Basic") => self.spinner_basic().into_any_element(),
            ("spinner", "Sizes") => self.spinner_sizes().into_any_element(),
            ("stepper", "Basic") => self.stepper_basic(cx).into_any_element(),
            ("stepper", "WithTextField") => self.stepper_with_text_field(cx).into_any_element(),
            ("stepper", "Sizes") => self.stepper_sizes().into_any_element(),
            ("stepper", "Disabled") => self.stepper_disabled().into_any_element(),
            ("switch", "Basic") => self.switch_basic(cx).into_any_element(),
            ("switch", "Sizes") => self.switch_sizes().into_any_element(),
            ("switch", "Disabled") => self.switch_disabled().into_any_element(),
            ("text-field", "Basic") => self.text_field_basic().into_any_element(),
            ("text-field", "Labelled") => self.text_field_labelled().into_any_element(),
            ("text-field", "Placeholder") => self.text_field_placeholder().into_any_element(),
            ("text-field", "Sizes") => self.text_field_sizes().into_any_element(),
            ("text-field", "Disabled") => self.text_field_disabled().into_any_element(),
            _ => self.button_basic().into_any_element(),
        }
    }
}
