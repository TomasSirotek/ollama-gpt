//! The chat panel: transcript, empty state, composer.

use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Background, Color, Element, Length};

use crate::app::{App, ChatMessage, Message};
use crate::theme::{
    BG_COMPOSER, BG_MAIN, BG_RAISED, BG_SEND_IDLE, TEXT_PRIMARY, TEXT_SECONDARY, TEXT_TERTIARY,
    rounded,
};
use crate::ui::icon_button;
use crate::ui::icons::{self, icon};

pub fn view(app: &App) -> Element<'_, Message> {
    let top_bar = row![
        Space::new().width(Length::Fill),
        icon_button(icons::SPARK, Message::ToggleThinking),
    ]
    .padding([10, 14]);

    container(column![
        top_bar,
        container(body(app))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding([0, 24]),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(BG_MAIN)),
        ..Default::default()
    })
    .into()
}

fn body(app: &App) -> Element<'_, Message> {
    if app.messages.is_empty() {
        // Empty state: heading and composer sit together in the middle of the
        // panel, rather than the composer being pinned to the bottom.
        column![
            text("What can I help with?").size(28).color(TEXT_PRIMARY),
            composer(app),
        ]
        .spacing(26)
        .align_x(Alignment::Center)
        .max_width(760)
        .into()
    } else {
        let mut transcript = column![].spacing(18).width(Length::Fill);
        for message in &app.messages {
            transcript = transcript.push(bubble(message));
        }

        column![
            scrollable(transcript)
                .height(Length::Fill)
                .width(Length::Fill),
            composer(app),
        ]
        .spacing(18)
        .max_width(760)
        .into()
    }
}

/// The pill: attach, field, a Think toggle, and the send disc.
fn composer(app: &App) -> Element<'_, Message> {
    let ready = !app.input.trim().is_empty();

    container(
        row![
            icon_button(icons::PLUS, Message::Attach),
            text_input("Ask anything", &app.input)
                .on_input(Message::InputChanged)
                .on_submit(Message::Send)
                .size(15)
                .padding(0)
                .style(|_, _| text_input::Style {
                    // The pill behind it already carries the colour, so the
                    // field itself is transparent and borderless.
                    background: Background::Color(Color::TRANSPARENT),
                    border: rounded(0.0),
                    icon: TEXT_TERTIARY,
                    placeholder: TEXT_TERTIARY,
                    value: TEXT_PRIMARY,
                    selection: BG_SEND_IDLE,
                }),
            row![
                icon(icons::SPARK, 17.0, TEXT_SECONDARY),
                text("Think").size(14).color(TEXT_SECONDARY),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
            // Fills in once there is something to send - the only state change
            // in the composer.
            button(icon(
                icons::ARROW_UP,
                18.0,
                if ready { Color::BLACK } else { TEXT_SECONDARY },
            ))
            .padding(7)
            .on_press(Message::Send)
            .style(move |_, _| button::Style {
                background: Some(Background::Color(if ready {
                    TEXT_PRIMARY
                } else {
                    BG_SEND_IDLE
                })),
                text_color: if ready { Color::BLACK } else { TEXT_SECONDARY },
                border: rounded(16.0),
                ..Default::default()
            }),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([12, 14])
    .style(|_| container::Style {
        background: Some(Background::Color(BG_COMPOSER)),
        border: rounded(26.0),
        ..Default::default()
    })
    .into()
}

/// User messages sit right in their own surface; replies run full width with no
/// bubble, so the model's text reads as the page rather than as a message.
fn bubble(message: &ChatMessage) -> Element<'_, Message> {
    if message.from_user {
        row![
            Space::new().width(Length::Fill),
            container(text(message.body.clone()).size(15).color(TEXT_PRIMARY))
                .padding([10, 14])
                .style(|_| container::Style {
                    background: Some(Background::Color(BG_RAISED)),
                    border: rounded(18.0),
                    ..Default::default()
                }),
        ]
        .into()
    } else {
        text(message.body.clone())
            .size(15)
            .color(TEXT_PRIMARY)
            .into()
    }
}
