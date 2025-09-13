use iced::{Element, Theme};
use iced::widget::button;

#[derive(Debug,Clone)]
pub struct Settings
{
    section: Section,
}
impl Settings
{
    pub fn new() -> Settings {
        Self {
            section: Section::Main,
        }
    }
    pub fn view<'a>(&self) -> Element<'a,Message>
    {
        button("a")
            .on_press(Message::OpenSection(Section::Language))
            .into()
    }
    pub fn update(&mut self, message: Message) 
    {
        
    }
}
#[derive(Debug, Clone)]
enum Section
{
    Main,
    Language,
}
#[derive(Debug,Clone)]
pub enum Message
{
    OpenSection(Section),
}