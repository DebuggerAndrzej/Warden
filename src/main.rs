use iced::widget::text;
use iced::{Element, Sandbox, Settings, Theme};

fn main() -> iced::Result {
    Warden::run(Settings::default())
}

struct Warden;
#[derive(Debug)]
enum Message {}

impl Sandbox for Warden{
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Warden")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello Warden").into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}