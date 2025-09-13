use iced::Element;
use iced::widget::button;
use crate::ui::panel::time::Section::Main;

pub struct Time
{
    section: Section,
}
impl Time
{
    pub fn new() -> Time {
        Self {
            section: Main,
        }
    }
    pub fn view<'a>(&self) -> Element<'a, Message> {
        button("a").into()
    }
    pub fn update(&mut self, message: Message)
    {
        
    }
}
#[derive(Debug, Clone)]
enum Section
{
    Main,
    AddMedicationSection,
}
#[derive(Debug, Clone)]
pub enum Message
{
    OpenSection(Section),
    AddMedication,
}