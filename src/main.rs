mod states;
mod ui;
mod update;
use ui::view;
use states::message::Message;

use iced as ice;

use states::panel::Panel;
use crate::states::state::State;
use crate::ui::panel::settings::Settings;
use crate::update::loadpanel::load_panel;

fn main() {
    ice::run("a", update, view::view).expect("a");
}
fn update(state : &mut State, message: Message) {
    match message{
        Message::OpenTime => load_panel(state, &Panel::Time),
        Message::OpenManageMeds =>  load_panel(state, &Panel::ManageMeds),
        Message::OpenRecord => load_panel(state, &Panel::Record),
        Message::OpenSettings => load_panel(state, &Panel::Settings),
        Message::Settings(settings) => {
            state.settingsui.update(settings);
        }
    }   
}
