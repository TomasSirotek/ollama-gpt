//! How the app is drawn.
//!
//! `view` composes the two panels and owns nothing else. The panels live in
//! sibling files because they change for different reasons - the rail changes
//! when navigation does, the chat changes when the conversation does.

mod chat;
pub(crate) mod icons;
mod settings;
mod sidebar;

use iced::widget::{button, row, stack};
use iced::{Background, Color, Element};

use crate::app::{App, Message};
use crate::theme::{BG_RAISED, TEXT_SECONDARY, rounded};
use crate::ui::icons::icon;

pub fn view(app: &App) -> Element<'_, Message> {
    let shell = row![sidebar::view(app), chat::view(app)];

    // The sheet is stacked over the whole window rather than nested inside a
    // panel, so its backdrop can dim the rail too.
    if app.settings_open {
        stack![shell, settings::view()].into()
    } else {
        shell.into()
    }
}

/// A borderless icon button. Lives here rather than in either panel because
/// several use it - the rail header, the composer, the settings sheet.
pub(crate) fn icon_button<'a>(source: &'static str, on_press: Message) -> Element<'a, Message> {
    button(icon(source, 19.0, TEXT_SECONDARY))
        .padding(6)
        .on_press(on_press)
        .style(|_, status| button::Style {
            background: Some(Background::Color(match status {
                button::Status::Hovered => BG_RAISED,
                _ => Color::TRANSPARENT,
            })),
            text_color: TEXT_SECONDARY,
            border: rounded(7.0),
            ..Default::default()
        })
        .into()
}
