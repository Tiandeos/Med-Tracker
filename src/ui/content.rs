use crate::states::app::App;
use crate::states::message::Message;
use crate::states::panel::Panel;
use iced::widget::{column, container};
use iced::{Element, Fill};

pub fn main_content<'a>(state: &App) -> Element<'a, Message> {
    container(
        column![match &state.state.panel {
            Panel::Time => state.uistate.timeui.view().map(Message::Time),
            Panel::Record => state.uistate.recordui.view().map(Message::Record),
            Panel::ManageMeds => state.uistate.managemedsui.view().map(Message::ManageMeds),
            Panel::Settings => state.uistate.settingsui.view().map(Message::Settings),
        }]
        .padding(5),
    )
    .width(Fill)
    .height(Fill)
    .into()
}
