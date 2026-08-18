//! A macOS pop-up button and its menu.

use std::rc::Rc;

use gpui::{
    App, Anchor, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_base::{Button as BaseButton, Popover, PopoverState};

use crate::{
    CONTROL_TEXT_WEIGHT, MaccnAppearance, MacControlSize, control_height, control_radius,
    control_text_size, menu_check_mark, pop_up_chevron, rgba_f, theme::ThemeExt as _,
};

type SelectHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// An item in a pop-up button menu.
#[derive(Clone)]
pub enum MacPopUpButtonItem {
    /// A standard selectable item.
    Item {
        label: SharedString,
        disabled: bool,
        on_select: Option<SelectHandler>,
    },
    /// A menu separator.
    Separator,
}

impl MacPopUpButtonItem {
    /// Creates a selectable item with the given label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self::Item {
            label: label.into(),
            disabled: false,
            on_select: None,
        }
    }

    /// Marks the item as disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        if let Self::Item { disabled: d, .. } = &mut self {
            *d = disabled;
        }
        self
    }

    /// Sets the selection handler.
    pub fn on_select(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        if let Self::Item { on_select, .. } = &mut self {
            *on_select = Some(Rc::new(handler));
        }
        self
    }

    /// A menu separator.
    pub fn separator() -> Self {
        Self::Separator
    }
}

/// A macOS pop-up button that opens a menu.
#[derive(IntoElement)]
pub struct MacPopUpButton {
    id: ElementId,
    size: MacControlSize,
    disabled: bool,
    selected: Option<usize>,
    placeholder: SharedString,
    items: Vec<MacPopUpButtonItem>,
}

impl MacPopUpButton {
    /// Creates a pop-up button with a stable element identifier.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: MacControlSize::Regular,
            disabled: false,
            selected: None,
            placeholder: "Select".into(),
            items: Vec::new(),
        }
    }

    /// Sets the AppKit control size.
    pub fn size(mut self, size: MacControlSize) -> Self {
        self.size = size;
        self
    }

    /// Disables the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the index of the currently selected item.
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the placeholder shown when nothing is selected.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Appends an item.
    pub fn item(mut self, item: MacPopUpButtonItem) -> Self {
        self.items.push(item);
        self
    }

    /// Sets the menu items.
    pub fn items(mut self, items: impl IntoIterator<Item = MacPopUpButtonItem>) -> Self {
        self.items = items.into_iter().collect();
        self
    }
}

impl RenderOnce for MacPopUpButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let size = self.size;
        let disabled = self.disabled;
        let label = self
            .selected
            .and_then(|index| {
                self.items.get(index).and_then(|item| match item {
                    MacPopUpButtonItem::Item { label, .. } => Some(label.clone()),
                    MacPopUpButtonItem::Separator => None,
                })
            })
            .unwrap_or_else(|| self.placeholder.clone());

        let trigger = trigger(self.id.clone(), size, disabled, label, &theme);

        if disabled {
            return trigger.into_any_element();
        }

        let id = self.id.clone();
        let items = self.items;
        let selected = self.selected;
        Popover::new(self.id)
            .anchor(Anchor::BottomLeft)
            .overlay_closable(true)
            .trigger(trigger)
            .content(move |state, window, cx| {
                menu(id.clone(), size, items.clone(), selected, state, window, cx)
            })
            .into_any_element()
    }
}

fn trigger(
    id: ElementId,
    size: MacControlSize,
    disabled: bool,
    label: SharedString,
    theme: &crate::MaccnTheme,
) -> BaseButton {
    let height = control_height(size);
    let padding_start = match size {
        MacControlSize::ExtraLarge | MacControlSize::Large => 16.,
        MacControlSize::Regular => 12.,
        MacControlSize::Small => 8.,
        MacControlSize::Mini => 6.,
    };
    let padding_end = match size {
        MacControlSize::ExtraLarge => 12.,
        MacControlSize::Large | MacControlSize::Regular => 10.,
        MacControlSize::Small => 6.,
        MacControlSize::Mini => 5.,
    };
    let chevron_gap = match size {
        MacControlSize::ExtraLarge => 12.,
        MacControlSize::Large | MacControlSize::Regular => 8.,
        MacControlSize::Small | MacControlSize::Mini => 6.,
    };
    let min_width = match size {
        MacControlSize::ExtraLarge => 106.,
        MacControlSize::Large => 92.,
        MacControlSize::Regular => 84.,
        MacControlSize::Small => 72.,
        MacControlSize::Mini => 60.,
    };

    BaseButton::new((id, "trigger"))
        .disabled(disabled)
        .min_w(px(min_width))
        .h(px(height))
        .pl(px(padding_start))
        .pr(px(padding_end))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(chevron_gap))
        .rounded(px(control_radius(size)))
        .bg(theme.control_button_opaque)
        .text_color(if disabled {
            theme.label_disabled
        } else {
            theme.label
        })
        .text_size(px(control_text_size(size)))
        .font_weight(CONTROL_TEXT_WEIGHT)
        .active(|style| style.bg(theme.control_button_pressed_opaque))
        .focus_visible(|style| style.shadow(crate::focus_ring_shadow(theme.focus_ring)))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .overflow_hidden()
                .text_ellipsis()
                .child(label),
        )
        .child(pop_up_chevron(
            if disabled {
                theme.label_disabled
            } else {
                theme.label
            },
            7.,
            12.,
        ))
}

fn menu(
    id: ElementId,
    size: MacControlSize,
    items: Vec<MacPopUpButtonItem>,
    selected: Option<usize>,
    _state: &mut PopoverState,
    _window: &mut Window,
    cx: &mut gpui::Context<PopoverState>,
) -> gpui::AnyElement {
    let theme = crate::MaccnTheme::global(cx);
    let entity = cx.entity();
    let min_width = match size {
        MacControlSize::ExtraLarge => 96.,
        MacControlSize::Large => 82.,
        MacControlSize::Regular => 74.,
        MacControlSize::Small => 62.,
        MacControlSize::Mini => 50.,
    };
    let item_height = match size {
        MacControlSize::ExtraLarge | MacControlSize::Large | MacControlSize::Regular => 24.,
        MacControlSize::Small => 22.,
        MacControlSize::Mini => 20.,
    };
    let item_font = match size {
        MacControlSize::ExtraLarge | MacControlSize::Large | MacControlSize::Regular => 13.,
        MacControlSize::Small => 11.,
        MacControlSize::Mini => 10.,
    };

    div()
        .flex()
        .flex_col()
        .min_w(px(min_width))
        .p(px(4.))
        .rounded(px(10.))
        .bg(theme.menu_bg)
        .border_1()
        .border_color(theme.menu_rim)
        .shadow(vec![gpui_base::box_shadow(
            px(0.),
            px(5.),
            px(20.),
            px(0.),
            if theme.appearance == MaccnAppearance::Light {
                rgba_f(0, 0, 0, 0.2)
            } else {
                rgba_f(0, 0, 0, 0.5)
            },
        )])
        .children(items.into_iter().enumerate().map(move |(index, item)| {
            match item {
                MacPopUpButtonItem::Item {
                    label,
                    disabled,
                    on_select,
                } => {
                    let entity = entity.clone();
                    let is_selected = selected == Some(index);
                    BaseButton::new((id.clone(), index.to_string()))
                        .disabled(disabled)
                        .h(px(item_height))
                        .pl(px(6.))
                        .pr(px(10.))
                        .flex()
                        .items_center()
                        .gap(px(4.))
                        .rounded(px(5.))
                        .text_size(px(item_font))
                        .font_weight(CONTROL_TEXT_WEIGHT)
                        .text_color(theme.label)
                        .hover(|style| {
                            style.bg(theme.selection_bg).text_color(theme.label_on_accent)
                        })
                        .when_some(on_select, |button, handler| {
                            button.on_click(move |_: &ClickEvent, window, cx| {
                                handler(window, cx);
                                entity.update(cx, |state, cx| state.set_open(false, cx));
                            })
                        })
                        .child(
                            div()
                                .flex_none()
                                .w(px(16.))
                                .flex()
                                .items_center()
                                .justify_center()
                                .when(is_selected, |this| {
                                    this.child(menu_check_mark(theme.label, 10., 8.))
                                }),
                        )
                        .child(label)
                        .into_any_element()
                }
                MacPopUpButtonItem::Separator => div()
                    .w_full()
                    .h(px(1.))
                    .my(px(3.))
                    .bg(theme.separator_vibrant)
                    .into_any_element(),
            }
        }))
        .into_any_element()
}
