mod states;
mod ui;
mod update;

use states::message::Message;
use ui::view;

use iced as ice;

use crate::states::state::State;
use crate::update::loadpanel::load_panel;
use states::panel::Panel;

fn main() {
    ice::application("Med Tracker", update, view::view)
        .theme(view::theme)
        .run()
        .expect("a");
}
fn update(state: &mut State, message: Message) {
    match message {
        Message::OpenTime => load_panel(state, &Panel::Time),
        Message::OpenManageMeds => load_panel(state, &Panel::ManageMeds),
        Message::OpenRecord => load_panel(state, &Panel::Record),
        Message::OpenSettings => load_panel(state, &Panel::Settings),
        Message::Settings(settings) => state.settingsui.update(settings),
        Message::Time(time) => state.timeui.update(time),
        Message::Record(record) => state.recordui.update(record),
        Message::ManageMeds(managemeds) => state.managemedsui.update(managemeds),
    }
}
