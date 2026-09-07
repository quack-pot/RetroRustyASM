use iced::widget::button;
use iced::widget::text_editor;

pub struct State {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    Assemble,
}

impl State {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }

            Message::Assemble => {}
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        return iced::widget::column![
            iced::widget::row![button("Assemble").on_press(Message::Assemble)],
            text_editor(&self.content)
                .on_action(Message::Edit)
                .height(iced::Length::Fill),
        ]
        .spacing(8)
        .into();
    }
}
