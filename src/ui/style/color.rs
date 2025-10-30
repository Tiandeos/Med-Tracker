use iced::Color;

pub fn darken(color: Color, amount: f32) -> Color {
    let r = (color.r - amount).max(0.0);
    let g = (color.g - amount).max(0.0);
    let b = (color.b - amount).max(0.0);
    Color::from_rgb(r, g, b)
}
pub fn lighten(color: Color, amount: f32) -> Color {
    let r = (color.r + amount).min(1.0);
    let g = (color.g + amount).min(1.0);
    let b = (color.b + amount).min(1.0);
    Color::from_rgb(r, g, b)
}
