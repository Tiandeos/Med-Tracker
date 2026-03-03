mod application;
mod audio;
mod notify;
mod persistence;
mod ui;
mod update;

use application::message::Message;
use iced::{self as ice, Task};
use ui::view;

use crate::application::app::App;
use crate::audio::alarm::{play_alarm, stop_alarm};
use crate::notify::notification::send_alarm_notification;
use crate::update::alarm_dismiss::dismiss_expired_alarms;
use crate::update::loadpanel::load_panel;
use crate::update::time_check::{check_medication_schedule, check_new_day, update_time};
use application::panel::Panel;
use chrono;
use ui::panel::{alarm, time};

fn main() {
    ice::application(new, update, view::view)
        .theme(view::theme)
        .centered()
        .exit_on_close_request(false)
        .subscription(|state| {
            iced::Subscription::batch([
                update_time(state),
                iced::window::close_requests().map(Message::CloseRequested),
            ])
        })
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

fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::TimeCheck => {
            let old_date = state.medicationtracker.last_generation_date;
            check_new_day(&mut state.medicationtracker);
            if old_date != state.medicationtracker.last_generation_date {
                save(state);
            }
            let alarming_records = check_medication_schedule(&state.medicationtracker);
            let mut any_new = false;
            for record_id in alarming_records {
                let is_new = !state.uistate.alarmui.alarming_records.contains(&record_id);
                if is_new {
                    any_new = true;
                    if let Some(record) = state
                        .medicationtracker
                        .records
                        .iter()
                        .find(|r| r.id == record_id)
                    {
                        let med_name = state
                            .medicationtracker
                            .medications
                            .iter()
                            .find(|m| m.id == record.medication_id)
                            .map(|m| m.name.as_str())
                            .unwrap_or("Unknown");
                        let time_str = record
                            .time
                            .with_timezone(&chrono::Local)
                            .format("%H:%M")
                            .to_string();
                        send_alarm_notification(med_name, &time_str);
                    }
                }
                state.uistate.alarmui.add_alarming_record(record_id);
            }
            if any_new {
                play_alarm();
            }
            if state.uistate.alarmui.is_active() && state.state.panel != Panel::Alarm {
                state.state.switch_to_alarm();
            }

            let had_expired = dismiss_expired_alarms(
                &mut state.medicationtracker,
                &mut state.uistate.alarmui.alarming_records,
            );
            if had_expired {
                save(state);
            }
            if !state.uistate.alarmui.is_active() && state.state.panel == Panel::Alarm {
                stop_alarm();
                state.state.restore_previous_panel();
            }

            Task::none()
        }
        Message::CloseRequested(id) => {
            state.window_id = Some(id);
            iced::window::set_mode(id, iced::window::Mode::Hidden)
        }
        Message::TrayLeftClick => {
            if let Some(id) = state.window_id {
                Task::batch([
                    iced::window::set_mode(id, iced::window::Mode::Windowed),
                    iced::window::gain_focus(id),
                ])
            } else {
                Task::none()
            }
        }
        Message::Quit => iced::exit(),
        Message::OpenTime => {
            load_panel(state, &Panel::Time);
            state.uistate.timeui.set_section_to_main();
            Task::none()
        }
        Message::OpenManageMeds => {
            load_panel(state, &Panel::ManageMeds);
            Task::none()
        }
        Message::OpenRecord => {
            load_panel(state, &Panel::Record);
            Task::none()
        }
        Message::OpenSettings => {
            load_panel(state, &Panel::Settings);
            Task::none()
        }
        Message::Settings(settings) => {
            state.uistate.settingsui.update(settings);
            Task::none()
        }
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
            Task::none()
        }
        Message::Record(record) => {
            state.uistate.recordui.update(record);
            Task::none()
        }
        Message::ManageMeds(managemeds) => {
            state.uistate.managemedsui.update(managemeds);
            Task::none()
        }
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
                stop_alarm();
                state.state.restore_previous_panel();
            }
            if should_save {
                save(state);
            }
            Task::none()
        }
        Message::HideSidebar => Task::none(),
        Message::OpenSidebar => Task::none(),
    }
}
