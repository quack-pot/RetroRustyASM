use crate::core::message::Message;
use crate::panes::pane_kind::PaneKind;

const DEFAULT_FRAME_RATE_TARGET_MS: u64 = 1000 / 60; // 60 FPS

pub struct State {
    running: bool,
    panes: iced::widget::pane_grid::State<PaneKind>,
}

impl State {
    pub fn new() -> Self {
        let (mut panes, editor) = iced::widget::pane_grid::State::new(PaneKind::Editor);
        panes.split(
            iced::widget::pane_grid::Axis::Vertical,
            editor,
            PaneKind::Screen,
        );

        Self {
            running: true,
            panes,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick(_now) => {}

            Message::PaneResized(event) => {
                self.panes.resize(event.split, event.ratio);
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        return iced::widget::pane_grid(&self.panes, |_pane, kind, _is_maximized| {
            let content: iced::Element<'_, Message> = match kind {
                PaneKind::Editor => iced::widget::text("Editor").into(),
                PaneKind::Screen => iced::widget::text("Screen").into(),
            };

            iced::widget::pane_grid::Content::new(content)
        })
        .on_resize(10, Message::PaneResized)
        .into();
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        if self.running {
            return iced::time::every(std::time::Duration::from_millis(
                DEFAULT_FRAME_RATE_TARGET_MS,
            ))
            .map(Message::Tick);
        }

        return iced::Subscription::none();
    }
}
