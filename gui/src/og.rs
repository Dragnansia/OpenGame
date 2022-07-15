use iced::{executor, Application, Column, Command};

pub struct OpenGame {
    pub title: String,
}

impl Application for OpenGame {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                title: String::from("OpenGame"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, _: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new().into()
    }
}
