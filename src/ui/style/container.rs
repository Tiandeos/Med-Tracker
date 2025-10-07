use ice::widget::container::Style;
use ice::{Background, Theme};
use iced::{self as ice, Border};

pub fn container_panel(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        background: Some(Background::Color(palette.background.weak.color)),
        border: Border {
            color: palette.background.strong.color,
            width: 3.0,
            radius: ice::border::Radius {
                top_left: 30.0,
                top_right: 30.0,
                bottom_left: 30.0,
                bottom_right: 30.0,
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
