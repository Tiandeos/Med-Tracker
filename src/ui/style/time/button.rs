use iced::widget::button::{Status, Style};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

pub fn calendar_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let text_color = if !palette.is_dark {
        Color::from_rgb8(20, 20, 20)
    } else {
        Color::from_rgb8(220, 220, 220)
    };

    let background_color = match status {
        Status::Active => palette.secondary.strong.color,
        Status::Disabled => palette.secondary.base.color,
        Status::Hovered => palette.secondary.base.color,
        Status::Pressed => palette.secondary.weak.color,
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
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector { x: 0.01, y: 4.0 },
            blur_radius: 4.0,
        },
        text_color,
        ..Default::default()
    }
}
pub fn add_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let text_color = if !palette.is_dark {
        Color::from_rgb8(20, 20, 20)
    } else {
        Color::from_rgb8(220, 220, 220)
    };

    let background_color = match status {
        Status::Active => palette.secondary.strong.color,
        Status::Disabled => palette.secondary.strong.color,
        Status::Hovered => palette.secondary.base.color,
        Status::Pressed => palette.secondary.weak.color,
    };

    Style {
        background: Some(Background::Color(background_color)),
        border: Border {
            color: Color::BLACK,
            width: 1.0,
            radius: iced::border::Radius {
                top_left: 42.0,
                top_right: 42.0,
                bottom_right: 42.0,
                bottom_left: 42.0,
            },
        },
        text_color,
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector { x: 0.01, y: 4.0 },
            blur_radius: 4.0,
        },
        ..Default::default()
    }
}
