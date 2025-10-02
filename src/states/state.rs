use std::collections::HashMap;

use chrono::Datelike;

use crate::states::medication::medication::Medication;
use crate::states::medication::record::Record;
use crate::states::medication::schedule::Schedule;
use crate::states::medication::{self, schedule};
use crate::states::panel::Panel;
use crate::states::settings::Settings;
pub struct State {
    pub panel: Panel,
    pub settings: Settings,
    pub medications: Vec<Medication>,
    pub records: HashMap<usize, Vec<Record>>,
}
impl State {
    pub fn new() -> Self {
        State {
            panel: Panel::Time,
            settings: Settings::new(),
            medications: Vec::new(),
            records: HashMap::new(),
        }
    }
    pub fn change_panel(&mut self, panel: &Panel) {
        self.panel = panel.clone();
    }
    pub fn set_records(&mut self) {
        let medications: &Vec<Medication> = &self.medications;
        if medications.is_empty() {
            return;
        }
        for medication in medications {
            let medication_index = self
                .medications
                .iter()
                .position(|m| m.name == medication.name)
                .unwrap();
            for schedule in &medication.schedule {
                let schedule_index = medication
                    .schedule
                    .iter()
                    .position(|s| s.time == schedule.time)
                    .unwrap();
                let weekday_list = &schedule.week_day;
                if schedule.period_type.is_some() {
                    // If periodic, check periodic schedule
                    //self.check_periodic_schedule();
                    continue;
                }
                let record: Record;
                if weekday_list.is_some() {
                    record = self.check_weekday(schedule, &medication_index);
                } else {
                    record = Record::empty_new();
                }
                if self.records.contains_key(&medication_index) {
                    let record_list = self.records.get_mut(&medication_index).unwrap();
                    record_list.push(record);
                } else {
                    let mut record_list: Vec<Record> = Vec::new();
                    record_list.push(record);
                    self.records.insert(medication_index, record_list);
                }
            }
        }
    }
    //fn check_periodic_schedule(&self) -> Record {}
    fn check_weekday(&self, schedule: &Schedule, medication_index: &usize) -> Vec<Record> {
        let records: Vec<Record> = Vec::new();
        if self.records.is_empty() {
            let year = chrono::Local::now().year() as u8;
            let month = chrono::Local::now().month() as u8;
            let day = chrono::Local::now().day() as u8;
            let hour = schedule.time[0];
            let minute = schedule.time[1];
            let time: [u8; 5] = [year, month, day, hour, minute];
            for weekday in schedule.week_day.as_ref().unwrap() {
                if chrono::Local::now().weekday() == *weekday {
                    let record: Record = Record::new(0, time);
                    return vec![record];
                }
            }
            return records;
        } else {
            let record_list = self.records.get(medication_index).unwrap();
            let last_record = record_list.last().unwrap();
            let last_time = last_record.time;
            let current_time: [u8; 5] = [
                chrono::Local::now().year() as u8,
                chrono::Local::now().month() as u8,
                chrono::Local::now().day() as u8,
                schedule.time[0],
                schedule.time[1],
            ];
            if last_time[0] == current_time[0]
                && last_time[1] == current_time[1]
                && last_time[2] == current_time[2]
                && last_time[3] == current_time[3]
                && last_time[4] == current_time[4]
            {
                // If the last record is the same as the current time, do not add a new record
                return records;
            } else {
                let record: Record = Record::new(0, current_time);
                return records;
            }
            return records;
        }
    }
    pub fn clear_records(&mut self) {
        self.records.clear();
    }
    pub fn clear_medication(&mut self, medication_index: &usize) {
        if medication_index >= &self.medications.len() {
            println!("Medication index out of bounds !!");
            return;
        }
        self.records.remove(&medication_index);
    }
}
