use iced::Settings;
use iced::pure::Sandbox;
use iced::pure::widget::{Button, Text, Column, Container};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
}

struct Example {
    button_state: iced::button::State,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example {
            button_state: iced::button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Software Manager")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {}
        }
    }

    fn view(&self) -> iced::Element<'static, Message> {
        let content = Column::new()
            .push(Text::new("Software Manager"))
            .push(
                Button::new(&mut self.button_state, Text::new("Install"))
                    .on_press(Message::ButtonPressed),
            );

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
