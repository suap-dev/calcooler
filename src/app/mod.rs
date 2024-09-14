pub mod config;
pub mod keypad;

use iced::widget::{column, horizontal_space, row, text};
use iced::{Element, Sandbox, Theme};

use crate::app::keypad::button;
use crate::Message;

#[derive(Default)]
pub struct App {
    screen_text: String,
    insertion_mode: InsertionMode,
    integer_part: u128,
    fraction_part: u128,
    is_negative: bool,
}
#[derive(Default)]
enum InsertionMode {
    #[default]
    Integer,
    Fraction,
}
impl App {
    fn compose_screen_text(&mut self) {
        self.screen_text.clear();
        self.is_negative.then(|| self.screen_text.push('−'));
        self.screen_text.push_str(&self.integer_part.to_string());
        (self.fraction_part != 0).then(|| {
            self.screen_text.push(',');
            self.screen_text.push_str(&self.fraction_part.to_string());
        });
    }
}
impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            screen_text: "0".to_string(),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        "Calcooler - a cool calculator".to_string()
    }

    fn update(&mut self, message: Message) {
        println!("Message: {message:?}");
        match message {
            Message::Digit(digit) => {
                match self.insertion_mode {
                    // TODO: DRY
                    InsertionMode::Integer => {
                        self.integer_part = (self.integer_part * 10) + u128::from(digit);
                    }
                    InsertionMode::Fraction => {
                        self.fraction_part = (self.fraction_part * 10) + u128::from(digit);
                    }
                }
            }
            Message::Comma => self.insertion_mode = InsertionMode::Fraction,
            _ => (),
        }
        self.compose_screen_text();
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
