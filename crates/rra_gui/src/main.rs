mod core;

use crate::core::state::State;

fn theme(_: &State) -> iced::Theme {
    iced::Theme::Dark
}

fn main() -> iced::Result {
    return iced::application(State::new, State::update, State::view)
        .subscription(State::subscription)
        .theme(theme)
        .title("RetroRustyASM")
        .run();
}
