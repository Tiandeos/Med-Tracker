use iced::{Subscription, time};

use crate::application::{
    app::App, medication::medication::Medication, message::Message,
    states::medicationtracker::MedicationTracker,
};
use chrono::{Datelike, Local, Timelike, Weekday};

pub fn update_time(state: &App) -> Subscription<Message> {
    time::every(time::Duration::from_secs(30)).map(|_| Message::TimeCheck)
}
pub fn check_medication_schedule(state: &mut MedicationTracker) {
    let now = Local::now();
    let current_minutes = now.hour() as u16 * 60 + now.minute() as u16;
    let medication_list: &mut Vec<Medication> = &mut state.medications;
    for medication in medication_list {
        let schedule_list = &medication.schedules;
        if schedule_list.is_empty() {
            continue;
        }
        for schedule in schedule_list {
            let is_in_day = match &schedule.week_day {
                Some(weekday_list) => check_weekday(weekday_list),
                None => true,
            };
            let scheduled_minutes = schedule.time[0] as u16 * 60 + schedule.time[1] as u16;
            let elapsed = current_minutes.wrapping_sub(scheduled_minutes);
            if is_in_day && elapsed <= 15 {
                println!(
                    "ALARM: {} is due at {:02}:{:02} ({} min ago)",
                    medication.name, schedule.time[0], schedule.time[1], elapsed
                );
            }
        }
    }
}

fn check_weekday(weekday_list: &[Weekday]) -> bool {
    if weekday_list.is_empty() {
        return true;
    }
    weekday_list.contains(&Local::now().weekday())
}
