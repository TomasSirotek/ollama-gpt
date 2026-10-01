//! The chat panel: transcript, empty state, composer, suggestions.

use iced::widget::{
    Space, button, column, container, row, scrollable, stack, svg, text, text_input,
};
use iced::{Alignment, Background, Color, ContentFit, Element, Length, Padding};

use crate::app::{App, ChatMessage, Message};
use crate::theme::{
    BG_COMPOSER, BG_MAIN, BG_RAISED, BG_SEND_IDLE, TEXT_PRIMARY, TEXT_SECONDARY, TEXT_TERTIARY,
    rounded,
};
use crate::ui::dither;
use crate::ui::icon_button;
use crate::ui::icons::{self, icon};

/// The three starting points under the composer.
const SUGGESTIONS: [(&str, &str); 3] = [
    (icons::IMAGE, "Create an image or sticker"),
    (icons::PEN, "Write or edit"),
    (icons::GLOBE, "Search the web"),
];

/// `narrow` hides the rail entirely and puts a hamburger in the top bar.
pub fn view(app: &App, narrow: bool) -> Element<'_, Message> {
    let mut top_bar = row![].spacing(4).align_y(Alignment::Center);
    if narrow {
        top_bar = top_bar.push(icon_button(icons::MENU, Message::ToggleDrawer));
    }
    top_bar = top_bar
        .push(Space::new().width(Length::Fill))
        .push(icon_button(icons::SPARK, Message::ToggleThinking));

    let panel = column![
        top_bar.padding([10, 14]),
        container(body(app, narrow))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            // Both widths need a bottom inset. Once a conversation exists the
            // composer is the last child, so without one it sits flush on the
            // window edge and reads as cut off rather than placed. The empty
            // state is centred and barely notices.
            .padding(if narrow {
                Padding {
                    top: 0.0,
                    right: 14.0,
                    bottom: 14.0,
                    left: 14.0,
                }
            } else {
                Padding {
                    top: 0.0,
                    right: 24.0,
                    bottom: 18.0,
                    left: 24.0,
                }
            }),
    ];

    // The halftone sits behind everything, and only before the first message -
    // it is a welcome, not a wallpaper.
    //
    // `push_under`, not `stack![backdrop, panel]`: a Stack takes its size from
    // its BASE layer, so a Fill-sized svg as the first child would drive the
    // layout instead of the content. The panel is the base; the backdrop goes
    // beneath it, constrained to the size the panel resolved to.
    let body: Element<'_, Message> = if app.messages.is_empty() {
        stack![panel]
            .push_under(
                svg(svg::Handle::from_memory(dither::BACKDROP.as_bytes()))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    // Contain is the default and letterboxes: the pattern kept
                    // its authored ratio and drew as a band across the middle
                    // of the panel, over the composer, instead of filling it.
                    // Cover rather than Fill - Fill stretches, which squashes
                    // the dots into streaks on a narrow window. Cover scales
                    // uniformly and crops the overflow, so dots stay round and
                    // the glow stays on the floor.
                    .content_fit(ContentFit::Cover),
            )
            .into()
    } else {
        panel.into()
    };

    container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(BG_MAIN)),
            ..Default::default()
        })
        .into()
}

fn body(app: &App, narrow: bool) -> Element<'_, Message> {
    if app.messages.is_empty() && narrow {
        // Narrow: no heading, and the composer sits on the bottom edge with the
        // suggestions stacked above it - thumbs are at the bottom of a narrow
        // window, and a centred block would leave the controls mid-screen.
        return column![
            Space::new().height(Length::Fill),
            suggestions(),
            composer(app, true),
        ]
        .spacing(14)
        .into();
    }

    if app.messages.is_empty() {
        // Wide: heading, composer and suggestions as one centred block.
        column![
            text("What can I help with?").size(28).color(TEXT_PRIMARY),
            composer(app, narrow),
            suggestions(),
        ]
        .spacing(22)
        .align_x(Alignment::Center)
        .max_width(740)
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
            composer(app, narrow),
        ]
        .spacing(18)
        .max_width(740)
        .into()
    }
}

/// Left-aligned list of starting points. Hidden once a conversation exists -
/// they are a way in, not a toolbar.
fn suggestions<'a>() -> Element<'a, Message> {
    let mut list = column![].spacing(2).width(Length::Fill);

    for (source, label) in SUGGESTIONS {
        list = list.push(
            button(
                row![
                    icon(source, 18.0, TEXT_SECONDARY),
                    text(label).size(14).color(TEXT_SECONDARY),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([9, 10])
            .on_press(Message::InputChanged(label.to_string()))
            .style(|_, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => BG_RAISED,
                    _ => Color::TRANSPARENT,
                })),
                text_color: TEXT_SECONDARY,
                border: rounded(10.0),
                ..Default::default()
            }),
        );
    }

    list.into()
}

/// The pill: attach, field, Think, mic, and the send disc. Think collapses to
/// its icon when the window is narrow, which is the first thing with slack.
fn composer(app: &App, narrow: bool) -> Element<'_, Message> {
    let ready = !app.input.trim().is_empty();

    let mut think = row![icon(icons::SPARK, 17.0, TEXT_SECONDARY)]
        .spacing(6)
        .align_y(Alignment::Center);
    if !narrow {
        think = think.push(text("Think").size(14).color(TEXT_SECONDARY));
    }

    let field = text_input("Ask anything", &app.input)
        .on_input(Message::InputChanged)
        .on_submit(Message::Send)
        .size(15)
        .padding(0)
        .style(|_, _| text_input::Style {
            // The pill behind it already carries the colour, so the field
            // itself is transparent and borderless.
            background: Background::Color(Color::TRANSPARENT),
            border: rounded(0.0),
            icon: TEXT_TERTIARY,
            placeholder: TEXT_TERTIARY,
            value: TEXT_PRIMARY,
            selection: BG_SEND_IDLE,
        });

    let send = button(icon(
        icons::ARROW_UP,
        18.0,
        if ready { Color::BLACK } else { TEXT_SECONDARY },
    ))
    .padding(7)
    .on_press(Message::Send)
    .style(move |_, _| button::Style {
        // Fills in once there is something to send - the only state change
        // in the composer.
        background: Some(Background::Color(if ready {
            TEXT_PRIMARY
        } else {
            BG_SEND_IDLE
        })),
        text_color: if ready { Color::BLACK } else { TEXT_SECONDARY },
        border: rounded(16.0),
        ..Default::default()
    });

    // Narrow: two rows, field above its controls, so the field keeps full
    // width instead of being squeezed between four buttons.
    let inner: Element<'_, Message> = if narrow {
        column![
            field,
            row![
                icon_button(icons::PLUS, Message::Attach),
                Space::new().width(Length::Fill),
                icon_button(icons::MIC, Message::Attach),
                send,
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .into()
    } else {
        row![
            icon_button(icons::PLUS, Message::Attach),
            field,
            think,
            icon_button(icons::MIC, Message::Attach),
            send,
        ]
        .spacing(10)
        .align_y(Alignment::Center)
        .into()
    };

    container(inner)
        .width(Length::Fill)
        .padding(if narrow { 14 } else { 10 })
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
