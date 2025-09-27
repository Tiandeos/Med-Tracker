mod states;
mod ui;
mod update;

use chrono::{Datelike, Local, Timelike};
use iced::{self as ice, Subscription, time};
use states::message::Message;
use ui::view;

use crate::states::app::App;
use crate::states::medication::medication::Medication;
use crate::states::state::State;
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
        Message::TimeCheck => check_medication_schedule(&mut state.state),
        Message::OpenTime => load_panel(state, &Panel::Time),
        Message::OpenManageMeds => load_panel(state, &Panel::ManageMeds),
        Message::OpenRecord => load_panel(state, &Panel::Record),
        Message::OpenSettings => load_panel(state, &Panel::Settings),
        Message::Settings(settings) => state.uistate.settingsui.update(settings),
        Message::Time(time) => state.uistate.timeui.update(&mut state.state, time),
        Message::Record(record) => state.uistate.recordui.update(record),
        Message::ManageMeds(managemeds) => state.uistate.managemedsui.update(managemeds),
    }
}
fn check_medication_schedule(state: &mut State) {
    let hour = Local::now().hour();
    let minute = Local::now().minute();
    let currentday = Local::now().weekday().to_string();
    println!("Hour: {} Minute: {} Weekday: {}", hour, minute, currentday);
    let medication_list: &mut Vec<Medication> = &mut state.medications;
    for medication in medication_list {
        println!("Medication Name:{}", medication.name);
        if hour >= medication.schedule[0].time[0] {
            println!("Medication hour: {}", medication.schedule[0].time[0]);
            if minute >= medication.schedule[0].time[1] {
                println!("Medication minute: {}", medication.schedule[0].time[1]);
            }
        }
    }
}
fn update_time(state: &App) -> Subscription<Message> {
    time::every(time::Duration::from_secs(1)).map(|_| Message::TimeCheck)
}
