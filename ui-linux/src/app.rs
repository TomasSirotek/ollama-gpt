//! State, events, and the one function that changes state.
//!
//! No widgets here. If you want to know what the app *is*, this file is the
//! whole answer; `ui/` only decides how to draw it.

use iced::Task;

/// A rail destination. Only New chat does anything yet; the rest are the shape
/// of the app, so the rail is not one lonely row.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Nav {
    NewChat,
    Models,
}

impl Nav {
    pub const ALL: [Nav; 2] = [Nav::NewChat, Nav::Models];

    /// Icon and label live with the variant rather than in the sidebar: adding
    /// a destination should be one edit, not three files.
    pub fn icon(self) -> &'static str {
        match self {
            Nav::NewChat => crate::ui::icons::PEN,
            Nav::Models => crate::ui::icons::BOX,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Nav::NewChat => "New chat",
            Nav::Models => "Models",
        }
    }
}

pub struct ChatMessage {
    pub from_user: bool,
    pub body: String,
}

pub struct App {
    pub nav: Nav,
    pub messages: Vec<ChatMessage>,
    pub input: String,
    /// The profile menu above the avatar row.
    pub menu_open: bool,
    /// The settings sheet over the whole window.
    pub settings_open: bool,
    /// Rail collapsed to an icon strip. Separate from the narrow-window case,
    /// which hides the rail entirely without touching this.
    pub sidebar_collapsed: bool,
    /// Narrow windows have no room for a rail, so it arrives as an overlay
    /// drawer instead. Separate from `sidebar_collapsed`: they are different
    /// controls answering different questions.
    pub drawer_open: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            nav: Nav::NewChat,
            messages: Vec::new(),
            input: String::new(),
            menu_open: false,
            settings_open: false,
            sidebar_collapsed: false,
            drawer_open: false,
        }
    }
}

/// Every event the app can produce. Adding a feature starts here: a new variant
/// makes `update` fail to compile until it is handled.
#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Send,
    Navigate(usize),
    Search,
    ToggleSidebar,
    ToggleDrawer,
    Attach,
    ToggleThinking,
    ToggleMenu,
    OpenSettings,
    CloseSettings,
    LogOut,
}

pub fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::InputChanged(value) => app.input = value,
        Message::Send => {
            let body = app.input.trim().to_string();
            if !body.is_empty() {
                app.messages.push(ChatMessage {
                    from_user: true,
                    body,
                });
                app.input.clear();
            }
        }
        Message::Navigate(index) => {
            app.nav = Nav::ALL[index];
            // Picking a destination is the end of the drawer's job.
            app.drawer_open = false;
            if app.nav == Nav::NewChat {
                app.messages.clear();
            }
        }
        Message::ToggleMenu => app.menu_open = !app.menu_open,
        Message::ToggleDrawer => app.drawer_open = !app.drawer_open,
        Message::ToggleSidebar => {
            app.sidebar_collapsed = !app.sidebar_collapsed;
            // A menu anchored to a row that is about to be 56px wide has
            // nowhere to go.
            app.menu_open = false;
        }
        Message::OpenSettings => {
            // Closing the menu as the sheet opens: leaving it behind the
            // backdrop means it is still there when the sheet closes.
            app.menu_open = false;
            app.settings_open = true;
        }
        Message::CloseSettings => app.settings_open = false,
        // Not wired yet. Listed rather than caught by `_` so adding behaviour
        // later means deleting a name from here, not hunting for a wildcard.
        Message::Search | Message::Attach | Message::ToggleThinking | Message::LogOut => {}
    }
    Task::none()
}
