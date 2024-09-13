use iced::widget::{container, horizontal_space, row, text};
use iced::{Element, Sandbox, Theme};

use crate::{config, Message};

pub struct App {
    screen_text: String,
}
impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            screen_text: "0".to_string(),
        }
    }

    fn title(&self) -> String {
        "Calcooler, a cooler calculator".to_string()
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        container(row![
            horizontal_space(),
            text(&self.screen_text).size(config::screen::TEXT_SIZE)
        ])
        .padding(config::app::PADDING)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}
