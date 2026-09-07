mod components;

use crate::components::panes::{editor, pane_kind::PaneKind, screen};
use iced::widget::pane_grid;

const DEFAULT_FRAME_RATE_TARGET: std::time::Duration = std::time::Duration::from_millis(1000 / 60); // 60 FPS

struct AppState {
    running: bool,
    panes: pane_grid::State<PaneKind>,

    editor: editor::State,
    screen: screen::State,
}

#[derive(Debug, Clone)]
enum AppMessage {
    Tick(std::time::Instant),

    PaneResized(pane_grid::ResizeEvent),

    Editor(editor::Message),
    Screen(screen::Message),
}

impl AppState {
    fn new() -> Self {
        let (mut panes, editor_pane) = pane_grid::State::new(PaneKind::Editor);
        panes.split(pane_grid::Axis::Vertical, editor_pane, PaneKind::Screen);

        return Self {
            running: true,
            panes,

            editor: editor::State::new(),
            screen: screen::State::new(100u32, 100u32),
        };
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Tick(_now) => {}

            AppMessage::PaneResized(event) => {
                self.panes.resize(event.split, event.ratio);
            }

            AppMessage::Editor(msg) => self.editor.update(msg),

            AppMessage::Screen(msg) => self.screen.update(msg),
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        return pane_grid(&self.panes, |_pane, kind, _is_maximized| {
            let content: iced::Element<'_, AppMessage> = match kind {
                PaneKind::Editor => self.editor.view().map(AppMessage::Editor),
                PaneKind::Screen => self.screen.view().map(AppMessage::Screen),
            };

            return pane_grid::Content::new(content);
        })
        .on_resize(10, AppMessage::PaneResized)
        .into();
    }

    fn subscription(&self) -> iced::Subscription<AppMessage> {
        if self.running {
            return iced::time::every(DEFAULT_FRAME_RATE_TARGET).map(AppMessage::Tick);
        }

        return iced::Subscription::none();
    }
}

fn main() -> iced::Result {
    return iced::application(AppState::new, AppState::update, AppState::view)
        .subscription(AppState::subscription)
        .title("RetroRustyASM")
        .run();
}
