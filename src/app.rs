use iced::Sandbox;

use crate::message::Message;

pub struct App {}
impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, _message: Self::Message) {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}
