use iced::{self as ice, Element, Length::Fill, Theme};

use crate::application::app::App;
use crate::application::message::Message;
use crate::ui::content::main_content;
use crate::ui::sidebar::{side_bar, sidebar_border};
use dark_light::Mode;
use ice::widget::row;

pub fn view(state: &App) -> Element<Message> {
    row![
        side_bar(&state.state.panel),
        sidebar_border(),
        main_content(state),
    ]
    .width(Fill)
    .height(Fill)
    .into()
}
pub fn theme(state: &App) -> Theme {
    if state.settings.is_theme_changed {
        state.settings.theme.clone()
    } else {
        detect_dark_light().unwrap_or(Theme::CatppuccinMocha)
    }
}
fn detect_dark_light() -> Result<Theme, dark_light::Error> {
    let mode = dark_light::detect()?;
    let theme = match mode {
        Mode::Dark => Theme::Nord,
        Mode::Light => Theme::Light,
        Mode::Unspecified => Theme::TokyoNightLight,
    };
    Ok(theme)
}
