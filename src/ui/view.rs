use iced::{self as ice, Element, Length::Fill, Theme};

use crate::states::app::App;
use crate::states::message::Message;
use crate::ui::content::main_content;
use crate::ui::sidebar::{side_bar, sidebar_border};
use dark_light::Mode;
use ice::widget::row;

pub fn view(state: &App) -> Element<Message> {
    row![side_bar(), sidebar_border(), main_content(&state),]
        .width(Fill)
        .height(Fill)
        .into()
}
pub fn theme(state: &App) -> Theme {
    if state.state.settings.is_theme_changed {
        state.state.settings.theme.clone()
    } else {
        detect_dark_light().unwrap_or(Theme::CatppuccinMocha)
    }
}
fn detect_dark_light() -> Result<Theme, dark_light::Error> {
    let mode = dark_light::detect()?;
    let theme: Theme;
    match mode {
        Mode::Dark => theme = Theme::Nord,
        Mode::Light => theme = Theme::Light,
        Mode::Unspecified => theme = Theme::TokyoNightLight,
    }
    Ok(theme)
}
