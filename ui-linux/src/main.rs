//! OllamaGPT - a native chat client for a locally running Ollama.
//!
//! This file is wiring only. The three modules below are the app:
//!
//!   app    what the app is    - state, events, and update
//!   ui     how it is drawn    - view, and one file per panel
//!   theme  what it looks like - colours and shared shapes
//!
//! Nothing talks to Ollama yet.

mod app;
mod theme;
mod ui;

use iced::Theme;

/// Iced wants a named function here, not a closure. A closure fails to satisfy
/// the higher-ranked lifetime bound with a confusing "implementation of `Fn` is
/// not general enough" error.
fn theme(_: &app::App) -> Theme {
    Theme::Dark
}

fn main() -> iced::Result {
    iced::application(app::App::default, app::update, ui::view)
        .title("OllamaGPT")
        .theme(theme)
        .run()
}
