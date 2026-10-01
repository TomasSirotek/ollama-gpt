//! Design tokens.
//!
//! Every colour and shared shape lives here, so retinting the app is one file -
//! the job `--color-*` did in the web version. Nothing here knows what a
//! sidebar or a composer is; it is values only.

use iced::{Border, Color};

/// The chat surface. Pure black, so the composer reads as the only object on it.
pub const BG_MAIN: Color = rgb(0x00, 0x00, 0x00);
/// The rail. Black, same as the chat - the two are separated by a hairline
/// rather than by a step in value.
pub const BG_SIDEBAR: Color = rgb(0x00, 0x00, 0x00);
/// The hairline between rail and chat. Light enough to read as an edge on
/// black, dim enough not to become a feature.
pub const BORDER_RAIL: Color = rgb(0x3a, 0x3a, 0x3a);
/// Selected nav row, hover, the avatar disc - anything lifting off the rail.
pub const BG_RAISED: Color = rgb(0x21, 0x21, 0x21);
/// The composer pill.
pub const BG_COMPOSER: Color = rgb(0x2f, 0x2f, 0x2f);
/// The send disc while there is nothing to send.
pub const BG_SEND_IDLE: Color = rgb(0x4a, 0x4a, 0x4a);
/// Floating surfaces - the profile menu and the settings sheet. A step above
/// the rail so they read as sitting on top of it.
pub const BG_MENU: Color = rgb(0x1c, 0x1c, 0x1c);
/// The hairline on those floating surfaces.
pub const BORDER_SUBTLE: Color = rgb(0x33, 0x33, 0x33);

pub const TEXT_PRIMARY: Color = rgb(0xec, 0xec, 0xec);
pub const TEXT_SECONDARY: Color = rgb(0xb4, 0xb4, 0xb4);
pub const TEXT_TERTIARY: Color = rgb(0x8f, 0x8f, 0x8f);

/// `Color::from_rgb8` is not const, so this is the const-friendly version.
const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

/// A corner radius with no visible stroke - the only border shape the app uses.
pub fn rounded(radius: f32) -> Border {
    Border {
        radius: radius.into(),
        ..Default::default()
    }
}
