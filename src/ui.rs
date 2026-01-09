use iced::{Application, executor, Settings, Command, Element};
use iced::widget::Text;

pub struct MyApp;

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (MyApp, Command::none())
    }

    fn title(&self) -> String {
        "TidaLuna Installer".into()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Text::new("Hello from GUI!").into()
    }
}

pub fn run_gui() -> iced::Result {
    MyApp::run(Settings::default())
}
