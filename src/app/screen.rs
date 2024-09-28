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
        self.is_text_full()?;

        self.text.clear();
        self.text.push_str(text);
        Ok(())
    }

    pub fn push_comma(&mut self) -> Result<(), Error> {
        if self.text.is_empty() {
            self.push_digit(0)?;
        }
        self.push('.')
    }

    pub fn push(&mut self, character: char) -> Result<(), Error> {
        self.is_text_full()?;

        self.text.push(character);
        Ok(())
    }

    pub fn push_digit(&mut self, digit: u32) -> Result<(), Error> {
        self.is_text_full()?;
        if self.text_is_integer_zero() {
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

    pub fn is_full(&self) -> bool {
        self.text.len() > config::screen::MAX_TEXT_LENGTH
    }

    fn is_text_full(&self) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::MaxInputReached)
        } else {
            Ok(())
        }
    }

    fn text_is_integer_zero(&self) -> bool {
        let mut chars = self.text.chars();
        chars.next().is_some_and(|value| value == '0') && chars.next().is_none()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    use super::*;
    #[test]
    fn the_zero_is_zero() {
        let mut screen = Screen::default();
        screen.push_digit(0);
        assert!(screen.text_is_integer_zero());
    }
    #[test]
    fn non_zero_is_not_zero() {
        let mut screen = Screen::default();
        screen.push_digit(1);
        assert!(!screen.text_is_integer_zero());
        screen.push_digit(0);
        assert!(!screen.text_is_integer_zero());
    }
    #[test]
    fn fraction_is_not_zero() {
        let mut screen = Screen::default();
        screen.push_digit(0);
        screen.push('.');
        assert!(!screen.text_is_integer_zero());
        screen.push_digit(0);
        assert!(!screen.text_is_integer_zero());
        screen.push_digit(1);
        assert!(!screen.text_is_integer_zero());
    }
    #[test]
    fn zero_to_zero_is_not_possible() {
        let mut screen = Screen::default();
        screen.push_digit(0);
        assert!(matches!(
            screen.push_digit(0).unwrap_err(),
            Error::PushingZeroToZero
        ));
    }
}
