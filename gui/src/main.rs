use iced::{Application, Settings};
use og::OpenGame;

pub mod og;

fn main() -> iced::Result {
    OpenGame::run(Settings::default())
}
