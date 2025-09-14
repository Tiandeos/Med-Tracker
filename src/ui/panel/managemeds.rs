use iced::Element;
use iced::widget::button;

pub struct ManageMedsUI
{
    section: Section
}
impl ManageMedsUI {
    pub fn new() -> Self {
        Self { section: Section::Main }
    }
    pub fn view<'a>(&self) -> Element<'a, Message>
    {
        button("c")
        .on_press(Message::OpenSection(Section::Main))
        .into()
    }
    pub fn update(&mut self, message: Message) {

    }
}

#[derive(Clone,Debug)]
pub enum Section
{
    Main,
}
#[derive(Clone,Debug)]
pub enum Message
{
    OpenSection(Section),
}