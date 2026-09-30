//! The settings sheet.
//!
//! A panel over a dimming backdrop, the same shape the web version used. The
//! backdrop is a full-size transparent button so a click anywhere outside the
//! panel closes it - Iced has no modal primitive, and this is what one is.

use iced::widget::{Space, button, column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use crate::app::Message;
use crate::theme::{
    BG_MENU, BG_RAISED, BORDER_SUBTLE, TEXT_PRIMARY, TEXT_SECONDARY, TEXT_TERTIARY, rounded,
};
use crate::ui::icon_button;
use crate::ui::icons;

pub fn view<'a>() -> Element<'a, Message> {
    let backdrop = button(Space::new().width(Length::Fill).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::CloseSettings)
        .style(|_, _| button::Style {
            background: Some(Background::Color(Color {
                a: 0.55,
                ..Color::BLACK
            })),
            ..Default::default()
        });

    let header = row![
        text("Settings").size(18).color(TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        icon_button(icons::CLOSE, Message::CloseSettings),
    ]
    .align_y(Alignment::Center);

    let panel = container(
        column![
            header,
            section("General"),
            setting_row("Model", "qwen3:4b"),
            setting_row("Ollama", "http://localhost:11434"),
            section("Appearance"),
            setting_row("Theme", "Dark"),
        ]
        .spacing(14),
    )
    .width(Length::Fixed(520.0))
    .padding(22)
    .style(|_| container::Style {
        background: Some(Background::Color(BG_MENU)),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: 18.0.into(),
        },
        ..Default::default()
    });

    // The panel is centred inside a container that fills the window, so the
    // backdrop underneath still receives clicks everywhere else.
    iced::widget::stack![
        backdrop,
        container(panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    ]
    .into()
}

fn section<'a>(label: &'a str) -> Element<'a, Message> {
    text(label).size(12).color(TEXT_TERTIARY).into()
}

/// A label on the left, its current value on the right. Read-only for now -
/// these become real controls once `core` owns the settings.
fn setting_row<'a>(label: &'a str, value: &'a str) -> Element<'a, Message> {
    container(
        row![
            text(label).size(14).color(TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text(value).size(13).color(TEXT_SECONDARY),
        ]
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([10, 12])
    .style(|_| container::Style {
        background: Some(Background::Color(BG_RAISED)),
        border: rounded(10.0),
        ..Default::default()
    })
    .into()
}
