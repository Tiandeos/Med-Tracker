use iced as ice;
use ice::widget::{button,button::Style};
use iced::{Background, Border, Color, Theme};
use iced::widget::button::Status;

pub fn navbar_button(theme: &Theme, status: Status) -> Style{
    match status {
        Status::Active =>  navbar_button_background(Color::from_rgb8(0,125,222),Color::from_rgb8(255,255,255)),
        Status::Disabled => navbar_button_background(Color::from_rgb8(22,22,22),Color::from_rgb8(255,255,255)),
        Status::Hovered => navbar_button_background(Color::from_rgb8(25,145,245),Color::from_rgb8(255,255,255)),
        Status::Pressed => navbar_button_background(Color::from_rgb8(0,100,200),Color::from_rgb8(255,255,255))
    }
}
fn navbar_button_background(background_color: Color, text_color: Color) -> Style {
    Style {
        background: Some(Background::Color(background_color)),
        text_color,
        ..Default::default()
    }
}