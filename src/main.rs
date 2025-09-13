mod states;
mod ui;
mod update;
use ui::view;
use states::message::Message;

use iced as ice;

use states::panel::Panel;
use crate::states::state::State;
use crate::update::loadpanel::load_panel;

fn main() {
    ice::run("a", update, view::view).expect("a");
}
fn update(state : &mut State, message: Message) {
    match message{
        Message::OpenPanel(Panel::Time) => load_panel(state, &Panel::Time),
        Message::OpenPanel(Panel::ManageMeds) =>  load_panel(state, &Panel::ManageMeds),
        Message::OpenPanel(Panel::Record) => load_panel(state, &Panel::Record),
        Message::OpenPanel(Panel::Settings) => load_panel(state, &Panel::Settings),
    }   
}
