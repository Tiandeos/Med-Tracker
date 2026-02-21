use ice::ContentFit;
use ice::widget::{Image, button, container, row, text};
use iced::{self as ice, Background, Element, Fill, Length, alignment};

use crate::application::message::Message;
use crate::ui::macros;
use crate::ui::style::button::navbar_button;
use crate::ui::style::color::lighten;

pub fn side_bar() -> Element<'static, Message> {
    container(
        iced::widget::column![
            button(macros::button_with_icon!(
                "icons/hidepanel_icon.png",
                32,
                10
            ))
            .style(navbar_button)
            .padding(0)
            .on_press(Message::HideSidebar)
            .height(Length::Fixed(80.0))
            .width(Fill),
            button(macros::button_with_icon_text!("Home", "icons/home.png"))
                .style(navbar_button)
                .padding(0)
                .on_press(Message::OpenTime)
                .height(Length::FillPortion(1))
                .width(Fill),
            button(macros::button_with_icon_text!(
                "Calendar",
                "icons/calendar.png"
            ))
            .style(navbar_button)
            .padding(0)
            .on_press(Message::OpenRecord)
            .height(Length::FillPortion(1))
            .width(Fill),
            button(macros::button_with_icon_text!(
                "Medications",
                "icons/pill.png"
            ))
            .style(navbar_button)
            .padding(0)
            .on_press(Message::OpenManageMeds)
            .height(Length::FillPortion(1))
            .width(Fill),
            button(macros::button_with_icon_text!(
                "Settings",
                "icons/settings.png"
            ))
            .style(navbar_button)
            .padding(0)
            .on_press(Message::OpenSettings)
            .height(Length::FillPortion(1))
            .width(Fill)
        ]
        .spacing(1),
    )
    .style(sidebar_style)
    .width(Length::Fixed(162.0))
    .height(Fill)
    .into()
}
pub fn sidebar_border() -> Element<'static, Message> {
    container("")
        .width(1)
        .height(Fill)
        .style(sidebar_border_style)
        .into()
}
fn sidebar_border_style(theme: &ice::Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(lighten(
            palette.background.base.color,
            0.1,
        ))),
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
