use ice::widget::container::Style;
use ice::{Background, Color, Theme};
use iced::{self as ice, Border, Shadow, Vector};

pub fn alarm_panel_container(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        background: Some(Background::Color(palette.background.weak.color)),
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
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector { x: 0.0, y: 4.0 },
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}
