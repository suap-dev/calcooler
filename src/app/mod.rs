mod config;
mod keypad;
mod message;
mod operation;
mod screen;

// use crate::app::config::screen::MAX_TEXT_LENGTH;
// use crate::app::{keypad::button, message::Message, operation::Operation};

use message::Message;
use operation::Operation;

use iced::widget::column;
use iced::{Element, Sandbox, Theme};

// TODO Error propagation
#[derive(Default)]
pub struct App {
    screen: screen::Screen,

    integer_part: u128,
    fraction_part: u128,
    is_negative: bool,

    insertion_mode: InsertionMode,

    registry: Option<f64>,
    selected_operation: Option<Operation>,
}
#[derive(Default)]
enum InsertionMode {
    #[default]
    Integer,
    Fraction,
}
impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        config::app::TITLE.into()
    }

    fn update(&mut self, message: Message) {
        println!("Message: {message:?}");
        match message {
            Message::Digit(digit) => {
                self.screen
                    .push_digit(digit)
                    .unwrap_or_else(|e| println!("Error: {e:?}"));
            }
            // TODO Check if there is an edge case in which we try to insert comma but it doesn't go in, but the insertion_mode is set to Fraction anyway.
            // Is it even a bad thing if that happens?
            Message::Comma => match self.insertion_mode {
                InsertionMode::Integer => {
                    self.screen
                        .push_comma()
                        .unwrap_or_else(|e| println!("Error: {e:?}"));
                    self.insertion_mode = InsertionMode::Fraction;
                }
                InsertionMode::Fraction => println!("Comma already added"),
            },
            Message::Operation(operation) => {
                self.selected_operation = Some(operation);
            }
            _ => (),
        }
        if let Ok(number) = self.screen.get_text().parse::<f64>() {
            println!("Number: {number}");
        } else {
            println!("Invalid number");
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let screen = self.screen.get();
        let keypad = keypad::get();

        column![screen, keypad].padding(config::app::PADDING).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}
