use iced::{Element, Fill};
use iced::widget::{container, column, Container, button};
use crate::states::message::Message;
use crate::states::panel::Panel;
use crate::states::state::State;

pub fn main_content<'a>(state: &State) -> Element<'a,Message>
{
    container(
        column![
            match &state.panel
            {
                Panel::Time => state.settingsui.view().map(Message::Settings),
                Panel::Record => state.settingsui.view().map(Message::Settings),
                Panel::ManageMeds =>state.settingsui.view().map(Message::Settings),
                Panel::Settings => state.settingsui.view().map(Message::Settings),
            }
        ]
    )
    .width(Fill)
    .height(Fill)
    .into()
}