
use iced::{self as ice, Element, Length, Length::Fill};
use ice::widget::{button,text,row,container};

use crate::states::message::Message;

pub fn view(a : &String) -> Element<Message> {
    container(row![
        text(a).size(15).width(Length::FillPortion(1)),
        button("aaa").on_press(Message::OpenPanel).width(Length::FillPortion(2)).padding(100),
    ]
    .spacing(10)
    )
    .padding(20)
    .center_x(Fill)
    .center_y(Fill)
    .into()
    
    
}
