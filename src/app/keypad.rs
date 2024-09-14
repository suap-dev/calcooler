use crate::Message;
use iced::{widget, Length};

pub fn button(content: &str, message: Message) -> widget::Button<Message> {
    // let digit_str = digit.to_string().as_str();
    widget::button(content)
        .on_press(message)
        .width(Length::Fill)
        .height(Length::Fill)
}
