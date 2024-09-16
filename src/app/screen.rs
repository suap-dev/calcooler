use crate::app::{config, message::Message};
use iced::widget;

#[derive(Default)]
pub struct Screen {
    text: String,
}
#[derive(Debug)]
pub enum Error {
    MaxInputReached,
    PushingZeroToZero,
    InvalidDigit(u32),
}
impl Screen {
    pub fn get(&self) -> widget::Row<'static, Message> {
        widget::row![
            widget::horizontal_space(),
            widget::text(self.get_text()).size(config::screen::TEXT_SIZE)
        ]
        .padding(config::screen::PADDING)
    }
    pub fn get_text(&self) -> &str {
        if self.text.is_empty() {
            "0"
        } else {
            &self.text
        }
    }

    pub fn set_text(&mut self, text: &str) -> Result<(), Error> {
        self.check_text_full()?;

        self.text.clear();
        self.text.push_str(text);
        Ok(())
    }

    pub fn push(&mut self, character: char) -> Result<(), Error> {
        self.check_text_full()?;

        self.text.push(character);
        Ok(())
    }

    pub fn push_digit(&mut self, digit: u32) -> Result<(), Error> {
        self.check_text_full()?;
        if self.text.is_empty() && digit == 0 {
            return Err(Error::PushingZeroToZero);
        }
        match char::from_digit(digit, 10) {
            Some(digit) => {
                self.text.push(digit);
                Ok(())
            }
            None => Err(Error::InvalidDigit(digit)),
        }
    }

    pub fn is_screen_full(&self) -> bool {
        self.text.len() > config::screen::MAX_TEXT_LENGTH
    }

    fn check_text_full(&self) -> Result<(), Error> {
        if self.is_screen_full() {
            Err(Error::MaxInputReached)
        } else {
            Ok(())
        }
    }
}
