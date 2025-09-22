use iced::{self as ice, Element, Length::Fill, Theme};

use crate::states::message::Message;
use crate::states::state::State;
use crate::ui::content::main_content;
use crate::ui::sidebar::{side_bar, sidebar_border};
use dark_light::{Error, Mode};
use ice::widget::row;

pub fn view(state: &State) -> Element<Message> {
    row![side_bar(), sidebar_border(), main_content(&state),]
        .width(Fill)
        .height(Fill)
        .into()
}
pub fn theme(state: &State) -> Theme {
    if state.settings.is_theme_changed {
        state.settings.theme.clone()
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
