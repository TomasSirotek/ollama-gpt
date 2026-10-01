//! The rail: brand, navigation, profile and its menu.
//!
//! Two widths. Expanded is 262px with labels; collapsed is a 56px strip of
//! icons. The collapsed strip drops the labels and the menu rather than
//! shrinking them, because a 56px column has no room for either.

use iced::widget::{Space, button, column, container, row, stack, text};
use iced::{Alignment, Background, Color, Element, Length};

use crate::app::{App, Message, Nav};
use crate::theme::{
    BG_MENU, BG_RAISED, BG_SIDEBAR, BORDER_RAIL, BORDER_SUBTLE, TEXT_PRIMARY, TEXT_SECONDARY,
    TEXT_TERTIARY, rounded,
};
use crate::ui::icon_button;
use crate::ui::icons::{self, icon};

/// Width of the collapsed strip.
const RAIL: f32 = 56.0;
const WIDE: f32 = 262.0;

/// The rail as an overlay, for windows too narrow to hold it inline. Same
/// contents as the expanded rail, plus a close control and a dimmed backdrop
/// that dismisses on click.
pub fn drawer(app: &App) -> Element<'_, Message> {
    let backdrop = button(Space::new().width(Length::Fill).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::ToggleDrawer)
        .style(|_, _| button::Style {
            background: Some(Background::Color(Color {
                a: 0.5,
                ..Color::BLACK
            })),
            ..Default::default()
        });

    let panel = container(
        column![
            row![
                text("OllamaGPT").size(16).color(TEXT_PRIMARY),
                Space::new().width(Length::Fill),
                icon_button(icons::SEARCH, Message::Search),
                icon_button(icons::CLOSE, Message::ToggleDrawer),
            ]
            .spacing(2)
            .padding([4, 8])
            .align_y(Alignment::Center),
            nav(app),
            Space::new().height(Length::Fill),
            profile(app),
        ]
        .spacing(10),
    )
    .width(Length::Fixed(252.0))
    .height(Length::Fill)
    .padding(6)
    .style(|_| container::Style {
        background: Some(Background::Color(BG_SIDEBAR)),
        ..Default::default()
    });

    // The panel is pinned left; the backdrop fills everything behind it.
    stack![backdrop, row![panel, Space::new().width(Length::Fill)]].into()
}

pub fn view(app: &App) -> Element<'_, Message> {
    let collapsed = app.sidebar_collapsed;

    let content = if collapsed {
        // Every child is the same 31px square (19px icon + 6px padding), so a
        // single spacing value produces even gaps. The avatar is wrapped in a
        // same-height box for the same reason - a bare 28px disc would sit on a
        // different rhythm from the buttons above it.
        column![
            icon_button(icons::PANEL, Message::ToggleSidebar),
            icon_button(icons::PEN, Message::Navigate(0)),
            icon_button(icons::SEARCH, Message::Search),
            icon_button(icons::CHAT, Message::Navigate(1)),
            Space::new().height(Length::Fill),
            container(avatar())
                .width(Length::Fill)
                .align_x(Alignment::Center),
        ]
        .spacing(8)
        .align_x(Alignment::Center)
    } else {
        column![
            header(),
            nav(app),
            // Eats the space between the nav and the profile, which is what
            // pins the profile to the bottom.
            Space::new().height(Length::Fill),
            profile(app),
        ]
        .spacing(10)
    };

    let width = if collapsed { RAIL } else { WIDE };

    let surface = container(content)
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .padding(6)
        .style(|_| container::Style {
            background: Some(Background::Color(BG_SIDEBAR)),
            ..Default::default()
        });

    // A 1px column rather than a Border: borders apply to all four sides, and
    // only the edge against the chat should be visible.
    row![surface, divider()].into()
}

/// The hairline separating rail from chat.
fn divider<'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(BORDER_RAIL)),
            ..Default::default()
        })
        .into()
}

fn header<'a>() -> Element<'a, Message> {
    row![
        text("OllamaGPT").size(17).color(TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        icon_button(icons::SEARCH, Message::Search),
        icon_button(icons::PANEL, Message::ToggleSidebar),
    ]
    .spacing(2)
    .padding([4, 8])
    .align_y(Alignment::Center)
    .into()
}

fn nav(app: &App) -> Element<'_, Message> {
    let mut rows = column![].spacing(2);

    for (index, item) in Nav::ALL.iter().enumerate() {
        let selected = *item == app.nav;
        let tint = if selected {
            TEXT_PRIMARY
        } else {
            TEXT_SECONDARY
        };

        rows = rows.push(
            button(
                row![icon(item.icon(), 19.0, tint), text(item.label()).size(14)]
                    .spacing(11)
                    .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([8, 10])
            .on_press(Message::Navigate(index))
            .style(move |_, status| button::Style {
                background: Some(Background::Color(
                    if selected || matches!(status, button::Status::Hovered) {
                        BG_RAISED
                    } else {
                        Color::TRANSPARENT
                    },
                )),
                text_color: tint,
                border: rounded(8.0),
                ..Default::default()
            }),
        );
    }

    rows.into()
}

/// The 28px disc. `align_x`/`align_y` rather than `center_x`/`center_y`:
/// `center_x(length)` SETS the width, so passing `Fill` to centre the glyph
/// stretched the disc to the full height of the rail.
fn avatar<'a>() -> Element<'a, Message> {
    container(icon(icons::USER, 16.0, TEXT_SECONDARY))
        .width(Length::Fixed(28.0))
        .height(Length::Fixed(28.0))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(|_| container::Style {
            background: Some(Background::Color(BG_RAISED)),
            border: rounded(14.0),
            ..Default::default()
        })
        .into()
}

/// The avatar row, with its menu above it when open.
///
/// A plain column, not `stack!`. Stacking drew the menu *over* the row and hid
/// it - and there is nothing to gain from overlaying here: the rail already has
/// a `Fill` spacer above this, so a taller profile block just eats some of that
/// slack and pushes itself up. No overlay, nothing covered.
fn profile(app: &App) -> Element<'_, Message> {
    let row_button = button(
        row![
            avatar(),
            column![
                text("Developer nil").size(13).color(TEXT_PRIMARY),
                text("Free").size(11).color(TEXT_TERTIARY),
            ]
            .spacing(1),
            Space::new().width(Length::Fill),
            icon(icons::CHEVRON_UP, 16.0, TEXT_TERTIARY),
        ]
        .spacing(9)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([5, 7])
    .on_press(Message::ToggleMenu)
    .style(|_, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => BG_RAISED,
            _ => Color::TRANSPARENT,
        })),
        text_color: TEXT_PRIMARY,
        border: rounded(10.0),
        ..Default::default()
    });

    if !app.menu_open {
        return row_button.into();
    }

    let menu = container(
        column![
            menu_item(icons::SETTINGS, "Settings", Message::OpenSettings),
            menu_item(icons::LOGOUT, "Log out", Message::LogOut),
        ]
        .spacing(2),
    )
    .width(Length::Fill)
    .padding(6)
    .style(|_| container::Style {
        background: Some(Background::Color(BG_MENU)),
        border: iced::Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    });

    column![menu, row_button].spacing(6).into()
}

fn menu_item<'a>(source: &'static str, label: &'a str, on_press: Message) -> Element<'a, Message> {
    button(
        row![icon(source, 17.0, TEXT_SECONDARY), text(label).size(13)]
            .spacing(9)
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([7, 8])
    .on_press(on_press)
    .style(|_, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => BG_RAISED,
            _ => Color::TRANSPARENT,
        })),
        text_color: TEXT_PRIMARY,
        border: rounded(8.0),
        ..Default::default()
    })
    .into()
}
