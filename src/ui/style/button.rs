use ice::widget::button::Style;
use iced as ice;
use iced::widget::button::Status;
use iced::{Background, Color, Theme};

pub fn navbar_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let color: Color;
    if !palette.is_dark {
        color = Color::from_rgb8(20, 20, 20);
    } else {
        color = Color::from_rgb8(220, 220, 220);
    }
    match status {
        Status::Active => navbar_button_background(palette.primary.base.color, color),
        Status::Disabled => {
            navbar_button_background(palette.background.weak.color, palette.background.weak.color)
        }
        Status::Hovered => navbar_button_background(palette.primary.strong.color, color),
        Status::Pressed => navbar_button_background(palette.primary.weak.color, color),
    }
}
fn navbar_button_background(background_color: Color, text_color: Color) -> Style {
    Style {
        background: Some(Background::Color(background_color)),
        text_color,
        ..Default::default()
    }
}
