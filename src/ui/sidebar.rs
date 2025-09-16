use ice::ContentFit;
use ice::widget::{Image, button, container, row, text};
use iced::{self as ice, Background, Color, Element, Fill, Length, alignment};

use crate::states::message::Message;
use crate::ui::style::button::navbar_button;

macro_rules! button_with_icon {
($label:expr, $icon_path:expr) => {
        container(
            row![
                Image::new($icon_path)
                    .content_fit(ContentFit::Cover)
                    .width(40)
                    .height(40),
                text($label).size(12)
            ].spacing(10)
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
        .spacing(4),
    )
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb8(0, 85, 175))),
        ..Default::default()
    })
    .width(Length::Fixed(150.0))
    .height(Fill)
    .into()
}
pub fn sidebar_border() -> Element<'static, Message> {
    container("")
        .width(4)
        .height(Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(0, 85, 175))),
            ..Default::default()
        })
        .into()
}
