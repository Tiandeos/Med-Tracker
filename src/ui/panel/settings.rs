use crate::ui::macros;
use crate::ui::style::button::bordered_button;
use iced::ContentFit;
use iced::Element;
use iced::Length::Fill;
use iced::alignment;
use iced::widget::{Image, button, column, container, row, scrollable, text};

#[derive(Debug, Clone)]
pub struct Settingsui {
    section: Section,
}
impl Settingsui {
    pub fn new() -> Settingsui {
        Self {
            section: Section::Main,
        }
    }
    pub fn view<'a>(&self) -> Element<'a, Message> {
        scrollable(
            column![
                button(macros::button_with_icon!("Language", "icons/home.png"))
                    .on_press(Message::OpenSection(Section::Language))
                    .width(Fill)
                    .height(100)
                    .style(bordered_button),
                button(macros::button_with_icon!("Theme", "icons/home.png"))
                    .on_press(Message::OpenSection(Section::Theme))
                    .width(Fill)
                    .height(100)
                    .style(bordered_button),
                button(macros::button_with_icon!("Sound", "icons/home.png"))
                    .on_press(Message::OpenSection(Section::Sound))
                    .width(Fill)
                    .height(100)
                    .style(bordered_button),
                button(macros::button_with_icon!("System", "icons/home.png"))
                    .on_press(Message::OpenSection(Section::System))
                    .width(Fill)
                    .height(100)
                    .style(bordered_button),
            ]
            .spacing(50),
        )
        .width(Fill)
        .height(Fill)
        .into()
    }
    pub fn update(&mut self, message: Message) {}
}
#[derive(Debug, Clone)]
enum Section {
    Main,
    Language,
    Theme,
    Sound,
    System,
}
#[derive(Debug, Clone)]
pub enum Message {
    OpenSection(Section),
}
