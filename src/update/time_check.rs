use iced::{Subscription, time};

use crate::application::{
    app::App, medication::occurrencestatus::OccurrenceStatus, message::Message,
    states::medicationtracker::MedicationTracker,
};
use crate::update::generate_records::generate_future_records;
use chrono::{Local, Timelike, Utc};

pub fn update_time(state: &App) -> Subscription<Message> {
    time::every(time::Duration::from_secs(30)).map(|_| Message::TimeCheck)
}

pub fn check_new_day(tracker: &mut MedicationTracker) {
    let today = Local::now().date_naive();
    let is_new_day = match tracker.last_generation_date {
        Some(last_date) => today > last_date,
        None => true,
    };
    if is_new_day {
        tracker.last_generation_date = Some(today);
        generate_future_records(tracker);
        println!("New day detected: {today}. Generated future records.");
    }
}

pub fn check_medication_schedule(tracker: &mut MedicationTracker) {
    let now = Utc::now();
    let alarm_window = 15;
    let current_minutes = now.hour() as u16 * 60 + now.minute() as u16;
    for record in &tracker.records {
        let is_pending = matches!(record.occurrence_status, OccurrenceStatus::Pending);
        if !is_pending {
            continue;
        }
        let scheduled_minutes = record.time.hour() as u16 * 60 + record.time.minute() as u16;
        let elapsed = current_minutes.wrapping_sub(scheduled_minutes);
        println!("Elapsed {}", elapsed);
        if elapsed <= alarm_window {
            let med_name = tracker
                .medications
                .iter()
                .find(|m| m.id == record.medication_id)
                .map(|m| m.name.as_str())
                .unwrap_or("Unknown");

            let local_time = record.time.with_timezone(&Local);
            println!(
                "ALARM: {} is due at {} ({} min ago)",
                med_name,
                local_time.format("%H:%M"),
                elapsed
            );
        }
    }
}
