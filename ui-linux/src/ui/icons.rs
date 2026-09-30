//! The icon set.
//!
//! Stroke SVGs on a 24x24 grid, embedded as strings rather than loaded from
//! disk so the binary stays self-contained - no asset directory to ship or
//! path to resolve at runtime.
//!
//! Every icon uses `stroke="currentColor"`, which lets `svg::Style` recolour it
//! per use. One file per colour would be the alternative, and a bad one.

use iced::widget::{Svg, svg};
use iced::{Color, Length};

const HEAD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#;

/// Magnifier.
pub const SEARCH: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<circle cx="11" cy="11" r="7"/><path d="M20 20l-4.3-4.3"/></svg>"#
);

/// Panel with a divider - collapse the rail.
pub const PANEL: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<rect x="3" y="4" width="18" height="16" rx="2"/><path d="M9 4v16"/></svg>"#
);

/// Pencil over a page - new chat.
pub const PEN: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<path d="M11 4H6a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5"/>"#,
    r#"<path d="M17.5 3.5a2.1 2.1 0 0 1 3 3L12 15l-4 1 1-4Z"/></svg>"#
);

/// A box in isometric - the model list.
pub const BOX: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<path d="M21 8.5v7a1.6 1.6 0 0 1-.8 1.4l-7.4 4a1.6 1.6 0 0 1-1.6 0l-7.4-4A1.6 1.6 0 0 1 3 15.5v-7"/>"#,
    r#"<path d="M3.3 7.6 11.2 3.3a1.6 1.6 0 0 1 1.6 0l7.9 4.3-8.7 4.7Z"/><path d="M12 12.3V21"/></svg>"#
);

pub const PLUS: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<path d="M12 5v14M5 12h14"/></svg>"#
);

pub const ARROW_UP: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.1" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<path d="M12 19V6M6 12l6-6 6 6"/></svg>"#
);

/// Four-point star - the thinking toggle.
pub const SPARK: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<path d="M12 3c.6 4.2 2.2 5.8 6.4 6.4-4.2.6-5.8 2.2-6.4 6.4-.6-4.2-2.2-5.8-6.4-6.4C9.8 8.8 11.4 7.2 12 3Z"/>"#,
    r#"<path d="M18 16c.3 1.8 1 2.5 2.8 2.8-1.8.3-2.5 1-2.8 2.8-.3-1.8-1-2.5-2.8-2.8 1.8-.3 2.5-1 2.8-2.8Z"/></svg>"#
);

pub const SETTINGS: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<circle cx="12" cy="12" r="3.2"/>"#,
    r#"<path d="M19.4 14a1.4 1.4 0 0 0 .3 1.5l.1.1a1.7 1.7 0 1 1-2.4 2.4l-.1-.1a1.4 1.4 0 0 0-1.5-.3 1.4 1.4 0 0 0-.9 1.3v.2a1.7 1.7 0 0 1-3.4 0v-.1a1.4 1.4 0 0 0-1-1.3 1.4 1.4 0 0 0-1.5.3l-.1.1a1.7 1.7 0 1 1-2.4-2.4l.1-.1a1.4 1.4 0 0 0 .3-1.5 1.4 1.4 0 0 0-1.3-.9H5a1.7 1.7 0 0 1 0-3.4h.1a1.4 1.4 0 0 0 1.3-1 1.4 1.4 0 0 0-.3-1.5l-.1-.1a1.7 1.7 0 1 1 2.4-2.4l.1.1a1.4 1.4 0 0 0 1.5.3H10a1.4 1.4 0 0 0 .9-1.3V5a1.7 1.7 0 0 1 3.4 0v.1a1.4 1.4 0 0 0 .9 1.3 1.4 1.4 0 0 0 1.5-.3l.1-.1a1.7 1.7 0 1 1 2.4 2.4l-.1.1a1.4 1.4 0 0 0-.3 1.5V10a1.4 1.4 0 0 0 1.3.9h.2a1.7 1.7 0 0 1 0 3.4h-.1a1.4 1.4 0 0 0-1.3.9Z"/></svg>"#
);

pub const LOGOUT: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<path d="M9 21H6a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3"/><path d="M16 17l5-5-5-5"/><path d="M21 12H9"/></svg>"#
);

pub const CLOSE: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<path d="M18 6 6 18M6 6l12 12"/></svg>"#
);

pub const USER: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<circle cx="12" cy="8.5" r="3.7"/><path d="M4.8 20a7.4 7.4 0 0 1 14.4 0"/></svg>"#
);

/// Chevron pointing up - the profile menu opens upward, being at the bottom.
pub const CHEVRON_UP: &str = concat!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">"#,
    r#"<path d="m6 15 6-6 6 6"/></svg>"#
);

/// Renders one of the constants above at `size`, tinted with `color`.
pub fn icon<'a>(source: &'static str, size: f32, color: Color) -> Svg<'a> {
    svg(svg::Handle::from_memory(source.as_bytes()))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |_, _| svg::Style { color: Some(color) })
}

// Silences the unused warning for HEAD, kept as documentation of the shared
// prelude every icon above repeats.
const _: &str = HEAD;
