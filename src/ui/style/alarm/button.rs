use iced::widget::button::{Status, Style};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

pub fn alarm_action_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let text_color = if palette.is_dark {
        Color::from_rgb8(220, 220, 220)
    } else {
        Color::from_rgb8(20, 20, 20)
    };
    let background_color = match status {
        Status::Active | Status::Disabled => palette.secondary.strong.color,
        Status::Hovered => palette.secondary.base.color,
        Status::Pressed => palette.secondary.weak.color,
    };

    Style {
        background: Some(Background::Color(background_color)),
        text_color,
        border: Border {
            color: Color::BLACK,
            width: 1.0,
            radius: iced::border::Radius::from(45.0),
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector { x: 0.01, y: 4.0 },
            blur_radius: 4.0,
        },
        ..Default::default()
    }
}

pub fn alarm_take_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let text_color = if palette.is_dark {
        Color::from_rgb8(220, 220, 220)
    } else {
        Color::from_rgb8(20, 20, 20)
    };
    let background_color = match status {
        Status::Active | Status::Disabled => palette.success.strong.color,
        Status::Hovered => palette.success.base.color,
        Status::Pressed => palette.success.weak.color,
    };

    Style {
        background: Some(Background::Color(background_color)),
        text_color,
        border: Border {
            color: Color::BLACK,
            width: 1.0,
            radius: iced::border::Radius::from(45.0),
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector { x: 0.01, y: 4.0 },
            blur_radius: 4.0,
        },
        ..Default::default()
    }
}
