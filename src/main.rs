mod states;
mod view;

use states::message::Message;

use iced as ice;

use crate::states::panel::Panel;

fn main() {
    ice::run("a", update, view::view).expect("a");
}
fn update(a : &mut String, message: Message) {
    match message{
        Message::OpenPanel(Panel::Time) => a.push_str("a"),
        Message::OpenPanel(Panel::Stock) =>  a.push_str("as"),
        Message::OpenPanel(Panel::Record) => a.push_str("as"),
        Message::OpenPanel(Panel::Settings) => a.push_str("a"),
    }   
}
