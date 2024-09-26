use iced::widget::{column, container, row};
use iced::widget::text::State;
use iced::{Fill, Element};
use iced::Theme;

#[derive(Debug, Clone)]
pub enum Message {}

fn update() {
}

fn view<Message>(state: &State<P>) -> Element<Message> {
    container(
        column![
            "Top",
            row!["Left", "Right"].spacing(10),
            "Bottom"
        ]
        .spacing(10)
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

pub fn main() -> iced::Result {
    iced::application("GUI-PORTAGE", update, view)
        .theme(theme)
        .run()
}

fn theme(state: &State) -> Theme {
    Theme::TokyoNight
}
