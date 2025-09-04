use std::string;

mod message;
use message::Message;

use iced::{self as ice, Element, Length, Length::Fill};
use ice::widget::{button,text,column,row,container};

fn main() {
    ice::run("a", update, view);
}
fn update(a : &mut String, message: Message) {
    match message{
        Message::Aaa =>  a.push_str("as"),
    }   
}
fn view(a : &String) -> Element<Message> {
    container(row![
        text(a).size(15).width(Length::FillPortion(1)),
        button("aaa").on_press(Message::Aaa).width(Length::FillPortion(2)).padding(100),
    ]
    .spacing(10)
    )
    .padding(20)
    .center_x(Fill)
    .center_y(Fill)
    .into()
    
    
}
