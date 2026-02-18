use ice::widget::container::Style;
use ice::{Background, Color, Theme};
use iced::{self as ice, Border};

pub fn schedule_container(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        background: Some(Background::Color(palette.background.strong.color)),
        border: Border {
            color: Color::BLACK,
            width: 1.0,
            radius: ice::border::Radius {
                top_left: 45.0,
                top_right: 45.0,
                bottom_left: 45.0,
                bottom_right: 45.0,
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
