use iced::widget::pane_grid;

const DEFAULT_FRAME_RATE_TARGET: std::time::Duration = std::time::Duration::from_millis(1000 / 60); // 60 FPS

struct AppState {
    running: bool,
    panes: pane_grid::State<i32>,
}

enum AppMessage {
    Tick(std::time::Instant),
}

impl AppState {
    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(0);

        return Self {
            running: true,
            panes,
        };
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Tick(_now) => {}
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        return pane_grid(&self.panes, |_pane, _kind, _is_maximized| {
            return iced::widget::text("Hello World!").into();
        })
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
