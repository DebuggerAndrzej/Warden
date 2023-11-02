use iced::executor;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Application, Command, Element, Length, Settings, Theme};
use std::io;
use std::path::Path;
use std::sync::Arc;

fn main() -> iced::Result {
    Warden::run(Settings::default())
}

struct Warden {
    content: String,
    error: Option<Error>,
    current_scroll_offset: scrollable::RelativeOffset,
}
#[derive(Debug, Clone)]
enum Message {
    FileOpened(Result<Arc<String>, Error>),
    Open,
    Scrolled(scrollable::Viewport),
}

impl Application for Warden {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: String::from("Hej"),
                error: None,
                current_scroll_offset: scrollable::RelativeOffset::START,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Warden")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Open => Command::perform(pick_file(), Message::FileOpened),
            Message::FileOpened(Ok(content)) => {
                self.content = content.to_string();
                Command::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::Scrolled(viewport) => {
                self.current_scroll_offset = viewport.relative_offset();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![button("open").on_press(Message::Open)];
        let file_content = text(&self.content);

        let scrollable_content = scrollable(file_content)
            .width(Length::Fill)
            .height(Length::Fill);

        container(column![controls, scrollable_content])
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn pick_file() -> Result<Arc<String>, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}
