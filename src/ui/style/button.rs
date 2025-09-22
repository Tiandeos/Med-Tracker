use ice::widget::button::Style;
use iced::widget::button::Status;
use iced::{self as ice, Border};
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
pub fn bordered_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let text_color: Color;
    if !palette.is_dark {
        text_color = Color::from_rgb8(20, 20, 20);
    } else {
        text_color = Color::from_rgb8(220, 220, 220);
    }
    match status {
        Status::Active => bordered_button_color(
            palette.primary.base.color,
            palette.background.weak.color,
            text_color,
        ),
        Status::Disabled => bordered_button_color(
            palette.background.strong.color,
            palette.background.weak.color,
            text_color,
        ),
        Status::Hovered => bordered_button_color(
            palette.primary.strong.color,
            palette.background.weak.color,
            text_color,
        ),
        Status::Pressed => bordered_button_color(
            palette.primary.weak.color,
            palette.background.weak.color,
            text_color,
        ),
    }
}
fn bordered_button_color(background_color: Color, border_color: Color, text_color: Color) -> Style {
    Style {
        background: Some(Background::Color(background_color)),
        border: Border {
            color: border_color,
            width: 2.0,
            radius: iced::border::Radius {
                top_left: (30.0),
                top_right: (30.0),
                bottom_right: (30.0),
                bottom_left: (30.0),
            },
        },
        text_color,
        ..Default::default()
    }
}
