mod states;

use states::message::Message;

use iced::{self as ice, Element, Length, Length::Fill};
use ice::widget::{button,text,row,container};

fn main() {
    ice::run("a", update, view).expect("a");
}
fn update(a : &mut String, message: Message) {
    match message{
        Message::OpenPanel =>  a.push_str("as"),
    }   
}
fn view(a : &String) -> Element<Message> {
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
