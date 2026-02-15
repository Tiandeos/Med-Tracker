mod application;
mod ui;
mod update;

use application::message::Message;
use iced::{self as ice};
use ui::view;

use crate::application::app::App;
use crate::update::loadpanel::load_panel;
use crate::update::time_check::{check_medication_schedule, check_new_day, update_time};
use application::panel::Panel;

fn main() {
    ice::application(new, update, view::view)
        .theme(view::theme)
        .centered()
        .subscription(update_time)
        .run()
        .expect("a");
}
fn new() -> App {
    let mut app = App::new();
    check_new_day(&mut app.medicationtracker);
    app
}
fn update(state: &mut App, message: Message) {
    match message {
        Message::TimeCheck => {
            check_new_day(&mut state.medicationtracker);
            check_medication_schedule(&mut state.medicationtracker);
        }
        Message::OpenTime => {
            load_panel(state, &Panel::Time);
            state.uistate.timeui.set_section_to_main();
        }
        Message::OpenManageMeds => load_panel(state, &Panel::ManageMeds),
        Message::OpenRecord => load_panel(state, &Panel::Record),
        Message::OpenSettings => load_panel(state, &Panel::Settings),
        Message::Settings(settings) => state.uistate.settingsui.update(settings),
        Message::Time(time) => state
            .uistate
            .timeui
            .update(&mut state.medicationtracker, time),
        Message::Record(record) => state.uistate.recordui.update(record),
        Message::ManageMeds(managemeds) => state.uistate.managemedsui.update(managemeds),
        Message::HideSidebar => println!("hide sidebar"),
        Message::OpenSidebar => println!("open sidebar"),
    }
}
