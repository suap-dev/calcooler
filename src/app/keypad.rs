use iced::{widget, Length};

use crate::app::message::Message;

pub fn button(content: &str, message: Message) -> widget::Button<Message> {
    widget::button(content)
        .on_press(message)
        .width(Length::Fill)
        .height(Length::Fill)
}
