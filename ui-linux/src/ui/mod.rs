//! How the app is drawn.
//!
//! `view` composes the two panels and owns nothing else. The panels live in
//! sibling files because they change for different reasons - the rail changes
//! when navigation does, the chat changes when the conversation does.

mod chat;
mod dither;
pub(crate) mod icons;
mod settings;
mod sidebar;

use iced::widget::{button, responsive, row, stack};
use iced::{Background, Color, Element};

use crate::app::{App, Message};
use crate::theme::{BG_RAISED, TEXT_SECONDARY, rounded};
use crate::ui::icons::icon;

/// Below this, the rail is dropped entirely and the chat gets a hamburger.
/// Collapsing it to the icon strip instead would still cost 56px of a window
/// this narrow.
const NARROW: f32 = 620.0;

pub fn view(app: &App) -> Element<'_, Message> {
    // `responsive` hands the available size to the closure, so the layout can
    // react to the window without the width living in App. Resizing is not a
    // state change the app needs to know about.
    let shell = responsive(move |size| {
        let narrow = size.width < NARROW;

        if !narrow {
            return row![sidebar::view(app), chat::view(app, false)].into();
        }

        // Narrow: no rail inline. The hamburger brings it back as an overlay
        // drawer over the chat.
        let panel = chat::view(app, true);
        if app.drawer_open {
            stack![panel, sidebar::drawer(app)].into()
        } else {
            panel
        }
    });

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
