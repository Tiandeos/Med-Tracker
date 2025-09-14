use iced::{Element, Fill};
use iced::widget::{container, column};
use crate::states::message::Message;
use crate::states::panel::Panel;
use crate::states::state::State;

pub fn main_content<'a>(state: &State) -> Element<'a,Message>
{
    container(
        column![
            match &state.panel
            {
                Panel::Time => state.timeui.view().map(Message::Time),
                Panel::Record => state.recordui.view().map(Message::Record),
                Panel::ManageMeds =>state.managemedsui.view().map(Message::ManageMeds),
                Panel::Settings => state.settingsui.view().map(Message::Settings),
            }
        ]
    )
    .width(Fill)
    .height(Fill)
    .into()
}