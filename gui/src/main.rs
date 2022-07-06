use core::OpenGame;
use iced::{Column, Element, Sandbox, Settings};

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for OpenGame {
    type Message = Message;

    fn new() -> Self {
        Styling::default()
    }

    fn title(&self) -> String {
        String::from("OpenGame")
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&mut self) -> Element<Message> {
        Column::new()
    }
}

fn main() -> iced::Result {
    OpenGame::run(Settings::default())
}
