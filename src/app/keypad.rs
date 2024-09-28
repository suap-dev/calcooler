use iced::{
    widget::{self, column, row},
    Length,
};

use crate::app::{config, message::Message, operation::Operation};

fn button(content: &str, message: Message) -> widget::Button<Message> {
    widget::button(content)
        .on_press(message)
        .width(Length::Fill)
        .height(Length::Fill)
}

pub fn get() -> widget::Column<'static, Message> {
    column![
        row![
            button("%", Message::Percent),
            button("CE", Message::ClearEntry),
            button("C", Message::Clear),
            button("Back", Message::Back),
        ]
        .spacing(config::keypad::BUTTON_SPACING),
        row![
            button("1/x", Message::Reciprocal),
            button("x²", Message::Square),
            button("√", Message::SquareRoot),
            button("÷", Message::Operation(Operation::Divide)),
        ]
        .spacing(config::keypad::BUTTON_SPACING),
        row![
            button("7", Message::Digit(7)),
            button("8", Message::Digit(8)),
            button("9", Message::Digit(9)),
            button("×", Message::Operation(Operation::Multiply)),
        ]
        .spacing(config::keypad::BUTTON_SPACING),
        row![
            button("4", Message::Digit(4)),
            button("5", Message::Digit(5)),
            button("6", Message::Digit(6)),
            button("−", Message::Operation(Operation::Subtract)),
        ]
        .spacing(config::keypad::BUTTON_SPACING),
        row![
            button("1", Message::Digit(1)),
            button("2", Message::Digit(2)),
            button("3", Message::Digit(3)),
            button("+", Message::Operation(Operation::Add)),
        ]
        .spacing(config::keypad::BUTTON_SPACING),
        row![
            button("±", Message::ToggleSign),
            button("0", Message::Digit(0)),
            button(".", Message::Comma),
            button("=", Message::Calculate),
        ]
        .spacing(config::keypad::BUTTON_SPACING)
    ]
    .spacing(config::keypad::BUTTON_SPACING)
}
