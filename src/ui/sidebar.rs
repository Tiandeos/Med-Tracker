use ice::ContentFit;
use ice::widget::{Image, button, container, row, text};
use iced::{self as ice, Background, Element, Fill, Length, alignment};

use crate::states::message::Message;
use crate::ui::style::button::navbar_button;

macro_rules! button_with_icon {
($label:expr, $icon_path:expr) => {
        container(
            row![
                Image::new($icon_path)
                    .content_fit(ContentFit::Cover)
                    .width(30)
                    .height(30),
                text($label).size(14).align_y(alignment::Vertical::Bottom)
            ].spacing(10).align_y(alignment::Vertical::Center)
        ).align_y(alignment::Vertical::Center)
    };
}
pub fn side_bar() -> Element<'static, Message> {
    container(
        iced::widget::column![
            button(button_with_icon!("Home", "icons/home.png"))
                .style(navbar_button)
                .padding(0)
                .on_press(Message::OpenTime)
                .height(Length::FillPortion(1))
                .width(Fill),
            button(button_with_icon!("Calendar", "icons/calendar.png"))
                .style(navbar_button)
                .padding(0)
                .on_press(Message::OpenRecord)
                .height(Length::FillPortion(1))
                .width(Fill),
            button(button_with_icon!("Medications", "icons/pill.png"))
                .style(navbar_button)
                .padding(0)
                .on_press(Message::OpenManageMeds)
                .height(Length::FillPortion(1))
                .width(Fill),
            button(button_with_icon!("Settings", "icons/settings.png"))
                .style(navbar_button)
                .padding(0)
                .on_press(Message::OpenSettings)
                .height(Length::FillPortion(1))
                .width(Fill)
        ]
        .spacing(1),
    )
    .style(sidebar_style)
    .width(Length::Fixed(130.0))
    .height(Fill)
    .into()
}
pub fn sidebar_border() -> Element<'static, Message> {
    container("")
        .width(3)
        .height(Fill)
        .style(sidebar_border_style)
        .into()
}
fn sidebar_border_style(theme: &ice::Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        ..Default::default()
    }
}
fn sidebar_style(theme: &ice::Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        ..Default::default()
    }
}
