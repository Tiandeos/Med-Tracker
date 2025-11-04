use iced::Element;
use iced::widget::button;

pub struct Record {
    section: Section,
}
impl Record {
    pub fn new() -> Record {
        Self {
            section: Section::Main,
        }
    }
    pub fn view<'a>(&self) -> Element<'a, Message> {
        button("b")
            .on_press(Message::OpenSection(Section::Main))
            .into()
    }
    pub fn update(&mut self, message: Message) {}
}
#[derive(Debug, Clone)]
pub enum Message {
    OpenSection(Section),
    CloseSection,
}
#[derive(Debug, Clone)]
pub enum Section {
    Main,
}
