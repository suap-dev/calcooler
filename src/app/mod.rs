pub(crate) mod config;
pub(crate) mod keypad;

use iced::widget::{column, horizontal_space, row, text};
use iced::{Element, Sandbox, Theme};

use crate::app::keypad::button;
use crate::Message;

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
        "Calcooler - a cooler calculator".to_string()
    }

    fn update(&mut self, message: Message) {
        println!("Message: {message:?}");
    }

    fn view(&self) -> Element<'_, Message> {
        let screen = row![
            horizontal_space(),
            text(&self.screen_text).size(config::screen::TEXT_SIZE)
        ]
        .padding(config::screen::PADDING);

        let keypad = {
            let row_1 = row![
                button("%", Message::Percent),
                button("CE", Message::ClearEntry),
                button("C", Message::Clear),
                button("Back", Message::Back),
            ]
            .spacing(config::keypad::BUTTON_SPACING);

            let row_2 = row![
                button("1/x", Message::Reciprocal),
                button("x²", Message::Square),
                button("√", Message::SquareRoot),
                button("÷", Message::Divide),
            ]
            .spacing(config::keypad::BUTTON_SPACING);

            let row_3 = row![
                button("7", Message::Digit(7)),
                button("8", Message::Digit(8)),
                button("9", Message::Digit(9)),
                button("×", Message::Multiply),
            ]
            .spacing(config::keypad::BUTTON_SPACING);

            let row_4 = row![
                button("4", Message::Digit(4)),
                button("5", Message::Digit(5)),
                button("6", Message::Digit(6)),
                button("−", Message::Subtract),
            ]
            .spacing(config::keypad::BUTTON_SPACING);

            let row_5 = row![
                button("1", Message::Digit(1)),
                button("2", Message::Digit(2)),
                button("3", Message::Digit(3)),
                button("+", Message::Add),
            ]
            .spacing(config::keypad::BUTTON_SPACING);

            let row_6 = row![
                button("±", Message::ToggleSign),
                button("0", Message::Digit(0)),
                button(",", Message::Comma),
                button("=", Message::Calculate),
            ]
            .spacing(config::keypad::BUTTON_SPACING);
            column![row_1, row_2, row_3, row_4, row_5, row_6]
                .spacing(config::keypad::BUTTON_SPACING)
        };

        column![screen, keypad].padding(config::app::PADDING).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}
