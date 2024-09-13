#![warn(clippy::pedantic)]

pub(crate) mod app;
pub(crate) mod config;
pub(crate) mod message;

pub use app::App;
pub use message::Message;
