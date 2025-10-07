mod states;
mod ui;
mod update;

use chrono::{Datelike, Local, Timelike, Weekday};
use iced::{self as ice};
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
    let month = Local::now().month() as u8;
    let day = Local::now().day() as u8;
    let hour = Local::now().hour() as u8;
    let minute = Local::now().minute() as u8;

    println!("Hour: {} Minute: {}", hour, minute);
    let medication_list: &mut Vec<Medication> = &mut state.medications;
    for medication in medication_list {
        println!("Medication Name:{}", medication.name);
        let schedule_list = &medication.schedule;
        if schedule_list.is_empty() {
            continue;
        }
        let mut is_in_day;
        for schedule in schedule_list {
            if schedule.is_completed {
                continue;
            }
            let weekday_list = &schedule.week_day;
            if weekday_list.is_some() {
                is_in_day = check_weekday(weekday_list.as_ref().unwrap());
            } else {
                is_in_day = true;
            }
            if is_in_day && hour >= schedule.time[0] && minute >= schedule.time[1] {
                println!("Medication hour: {}", medication.schedule[0].time[0]);
            }
        }
    }
}
fn check_weekday(weekday_list: &Vec<Weekday>) -> bool {
    let mut is_in_day: bool = false;
    if weekday_list.is_empty() {
        return true;
    }
    let currentday = Local::now().weekday();
    for weekday in weekday_list {
        if *weekday == currentday {
            is_in_day = true;
            break;
        } else {
            is_in_day = false;
        }
    }
    is_in_day
}

fn update_time(state: &App) -> Subscription<Message> {
    time::every(time::Duration::from_secs(1)).map(|_| Message::TimeCheck)
}
