mod core;
mod panes;

use crate::core::{state::State, theme::get_theme};

fn main() -> iced::Result {
    return iced::application(State::new, State::update, State::view)
        .subscription(State::subscription)
        .theme(get_theme)
        .title("RetroRustyASM")
        .run();
}
