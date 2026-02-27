mod application;
mod persistence;
mod ui;
mod update;

use application::message::Message;
use iced::{self as ice};
use ui::view;

use crate::application::app::App;
use crate::update::loadpanel::load_panel;
use crate::update::time_check::{check_medication_schedule, check_new_day, update_time};
use application::panel::Panel;
use ui::panel::{alarm, time};

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
    if let Some(tracker) = persistence::load_tracker() {
        app.medicationtracker = tracker;
    }
    let old_date = app.medicationtracker.last_generation_date;
    check_new_day(&mut app.medicationtracker);
    if old_date != app.medicationtracker.last_generation_date {
        save(&app);
    }
    app
}
fn save(state: &App) {
    if let Err(e) = persistence::save_tracker(&state.medicationtracker) {
        eprintln!("Save failed: {e}");
    }
}
fn update(state: &mut App, message: Message) {
    match message {
        Message::TimeCheck => {
            let old_date = state.medicationtracker.last_generation_date;
            check_new_day(&mut state.medicationtracker);
            if old_date != state.medicationtracker.last_generation_date {
                save(state);
            }
            let alarming_records = check_medication_schedule(&state.medicationtracker);
            for record_id in alarming_records {
                state.uistate.alarmui.add_alarming_record(record_id);
            }
            if state.uistate.alarmui.is_active() && state.state.panel != Panel::Alarm {
                state.state.switch_to_alarm();
            }
        }
        Message::OpenTime => {
            load_panel(state, &Panel::Time);
            state.uistate.timeui.set_section_to_main();
        }
        Message::OpenManageMeds => load_panel(state, &Panel::ManageMeds),
        Message::OpenRecord => load_panel(state, &Panel::Record),
        Message::OpenSettings => load_panel(state, &Panel::Settings),
        Message::Settings(settings) => state.uistate.settingsui.update(settings),
        Message::Time(msg) => {
            let should_save = matches!(
                msg,
                time::Message::AddMedication
                    | time::Message::MarkTaken(_)
                    | time::Message::MarkSkipped(_)
            );
            state
                .uistate
                .timeui
                .update(&mut state.medicationtracker, msg);
            if should_save {
                save(state);
            }
        }
        Message::Record(record) => state.uistate.recordui.update(record),
        Message::ManageMeds(managemeds) => state.uistate.managemedsui.update(managemeds),
        Message::Alarm(msg) => {
            let should_save = matches!(
                msg,
                alarm::Message::MarkTaken(_) | alarm::Message::MarkSkipped(_)
            );
            state
                .uistate
                .alarmui
                .update(&mut state.medicationtracker, msg);
            if !state.uistate.alarmui.is_active() {
                state.state.restore_previous_panel();
            }
            if should_save {
                save(state);
            }
        }
        Message::HideSidebar => println!("hide sidebar"),
        Message::OpenSidebar => println!("open sidebar"),
    }
}
