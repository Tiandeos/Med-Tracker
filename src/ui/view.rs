use iced::{self as ice, Element, Length::Fill};

use crate::states::message::Message;
use crate::states::state::State;
use crate::ui::content::main_content;
use crate::ui::sidebar::{side_bar, sidebar_border};
use ice::widget::row;

pub fn view(state: &State) -> Element<Message> {
    row![side_bar(), sidebar_border(), main_content(&state),]
        .width(Fill)
        .height(Fill)
        .into()
}
