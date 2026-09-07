use iced::widget::{image, mouse_area};

pub struct State {
    width: u32,
    height: u32,
    framebuffer: image::Handle,
}

#[derive(Debug, Clone)]
pub enum Message {
    Clicked,
}

impl State {
    pub fn new(width: u32, height: u32) -> Self {
        let blank = vec![0u8; (width * height * 4) as usize];

        return Self {
            width,
            height,
            framebuffer: image::Handle::from_rgba(width, height, blank),
        };
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Clicked => {}
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let screen = image(self.framebuffer.clone());

        return mouse_area(screen).on_press(Message::Clicked).into();
    }
}
