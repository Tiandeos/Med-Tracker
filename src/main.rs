mod states;
mod view;

use states::message::Message;

use iced as ice;

fn main() {
    ice::run("a", update, view::view).expect("a");
}
fn update(a : &mut String, message: Message) {
    match message{
        Message::OpenPanel =>  a.push_str("as"),
    }   
}
