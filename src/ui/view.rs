use iced::{self as ice, Background, Color, Element, Length::Fill};

use ice::widget::{button,row,column,container};
use iced::widget::Container;
use crate::states::{message::Message, panel::Panel};
use crate::states::state::State;
use crate::ui::sidebar::{side_bar, sidebar_border};

pub fn view(state : &State) -> Element<Message> {
    let main_content = container(
        column![
            match &state.panel
            {
                Panel::Time => Container::new(button("a")),
                Panel::Record => Container::new(button("b")),
                Panel::ManageMeds => Container::new(button("c")),
                Panel::Settings => Container::new(button("d"))
            }
        ]
    ).width(Fill)
    .height(Fill);
    row![
        side_bar(),
        sidebar_border(),
        main_content
    ].width(Fill)
    .height(Fill)
    .into()
}
