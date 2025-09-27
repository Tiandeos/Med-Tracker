mod states;
mod ui;
mod update;

use chrono::{Local, Timelike};
use iced::{self as ice, Subscription, time};
use states::message::Message;
use ui::view;

use crate::states::app::App;
use crate::update::loadpanel::load_panel;
use states::panel::Panel;

fn main() {
    ice::application("Med Tracker", update, view::view)
        .theme(view::theme)
        .subscription(update_time)
        .run()
        .expect("a");
}
fn update(state: &mut App, message: Message) {
    match message {
        Message::TimeCheck => check_medication_schedule(),
        Message::OpenTime => load_panel(state, &Panel::Time),
        Message::OpenManageMeds => load_panel(state, &Panel::ManageMeds),
        Message::OpenRecord => load_panel(state, &Panel::Record),
        Message::OpenSettings => load_panel(state, &Panel::Settings),
        Message::Settings(settings) => state.uistate.settingsui.update(settings),
        Message::Time(time) => state.uistate.timeui.update(time),
        Message::Record(record) => state.uistate.recordui.update(record),
        Message::ManageMeds(managemeds) => state.uistate.managemedsui.update(managemeds),
    }
}
fn check_medication_schedule() {
    let a = Local::now().to_utc().hour();
    println!("Time: {}", a);
}
fn update_time(state: &State) -> Subscription<Message> {
    time::every(time::Duration::from_secs(1)).map(|_| Message::TimeCheck)
}
