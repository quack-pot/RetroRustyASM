#[derive(Debug, Clone)]
pub enum Message {
    Tick(std::time::Instant),
    PaneResized(iced::widget::pane_grid::ResizeEvent),
}
