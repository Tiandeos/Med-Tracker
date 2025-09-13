use iced::{Element, Fill};
use iced::widget::{container, column, Container, button};
use crate::states::message::Message;
use crate::states::panel::Panel;
use crate::states::state::State;

pub fn main_content(state: &State) -> Element<'static,Message>
{
    container(
        column![
            match &state.panel
            {
                Panel::Time => Container::new(button("a")),
                Panel::Record => Container::new(button("b")),
                Panel::ManageMeds => Container::new(button("c")),
                Panel::Settings => Container::new(button("d"))
            }
        ]
    )
    .width(Fill)
    .height(Fill)
    .into()
}