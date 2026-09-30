//! The rail: brand, navigation, profile and its menu.

use iced::widget::{Space, button, column, container, row, stack, text};
use iced::{Alignment, Background, Color, Element, Length};

use crate::app::{App, Message, Nav};
use crate::theme::{
    BG_MENU, BG_RAISED, BG_SIDEBAR, BORDER_SUBTLE, TEXT_PRIMARY, TEXT_SECONDARY, TEXT_TERTIARY,
    rounded,
};
use crate::ui::icon_button;
use crate::ui::icons::{self, icon};

pub fn view(app: &App) -> Element<'_, Message> {
    container(
        column![
            header(),
            nav(app),
            // Eats the space between the nav and the profile, which is what
            // pins the profile to the bottom.
            Space::new().height(Length::Fill),
            profile(app),
        ]
        .spacing(10),
    )
    .width(Length::Fixed(262.0))
    .height(Length::Fill)
    .padding(6)
    .style(|_| container::Style {
        background: Some(Background::Color(BG_SIDEBAR)),
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

/// The avatar row, with its menu stacked above it when open.
///
/// `stack!` draws later children over earlier ones in the same space. The menu
/// is therefore laid out on top of the row rather than pushing it, which is
/// what makes it behave like a popover without needing a real overlay.
fn profile(app: &App) -> Element<'_, Message> {
    let row_button = button(
        row![
            container(icon(icons::USER, 16.0, TEXT_SECONDARY))
                .width(Length::Fixed(28.0))
                .height(Length::Fixed(28.0))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(|_| container::Style {
                    background: Some(Background::Color(BG_RAISED)),
                    border: rounded(14.0),
                    ..Default::default()
                }),
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

    // The spacer reserves the avatar row's height so the menu sits directly
    // above it rather than over it.
    stack![
        row_button,
        column![menu, Space::new().height(Length::Fixed(46.0)),],
    ]
    .into()
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
