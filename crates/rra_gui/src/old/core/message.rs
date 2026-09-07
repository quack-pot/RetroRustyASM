#[derive(Debug, Clone)]
pub enum Message {
    Tick(std::time::Instant),

    PaneDragged(iced::widget::pane_grid::DragEvent),
    PaneResized(iced::widget::pane_grid::ResizeEvent),
}
