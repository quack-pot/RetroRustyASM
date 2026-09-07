mod components;

use crate::components::panes::editor;
use crate::components::panes::pane_kind::PaneKind;
use iced::widget::pane_grid;

const DEFAULT_FRAME_RATE_TARGET: std::time::Duration = std::time::Duration::from_millis(1000 / 60); // 60 FPS

struct AppState {
    running: bool,
    panes: pane_grid::State<PaneKind>,

    editor: editor::State,
}

#[derive(Debug, Clone)]
enum AppMessage {
    Tick(std::time::Instant),

    PaneResized(pane_grid::ResizeEvent),

    Editor(editor::Message),
}

impl AppState {
    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(PaneKind::Editor);

        return Self {
            running: true,
            panes,

            editor: editor::State::new(),
        };
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Tick(_now) => {}

            AppMessage::PaneResized(event) => {
                self.panes.resize(event.split, event.ratio);
            }

            AppMessage::Editor(msg) => self.editor.update(msg),
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        return pane_grid(&self.panes, |_pane, kind, _is_maximized| {
            let content: iced::Element<'_, AppMessage> = match kind {
                PaneKind::Editor => self.editor.view().map(AppMessage::Editor),
            };

            return iced::widget::pane_grid::Content::new(content);
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
