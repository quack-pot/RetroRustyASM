pub struct State;

#[derive(Debug, Clone)]
pub enum Message {}

impl State {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> iced::Element<'_, Message> {
        iced::widget::text("RetroRustyASM").into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::none()
    }
}
