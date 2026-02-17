use iced::widget::button::{Status, Style};
use iced::{Background, Border, Color, Theme};

pub fn time_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let text_color = if !palette.is_dark {
        Color::from_rgb8(20, 20, 20)
    } else {
        Color::from_rgb8(220, 220, 220)
    };

    let background_color = match status {
        Status::Active => palette.background.weak.color,
        Status::Disabled => palette.background.strong.color,
        Status::Hovered => palette.primary.strong.color,
        Status::Pressed => palette.primary.weak.color,
    };

    Style {
        background: Some(Background::Color(background_color)),
        border: Border {
            color: Color::BLACK,
            width: 1.0,
            radius: iced::border::Radius {
                top_left: 60.0,
                top_right: 60.0,
                bottom_right: 60.0,
                bottom_left: 60.0,
            },
        },
        text_color,
        ..Default::default()
    }
}
