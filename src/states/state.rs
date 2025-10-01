use chrono::{Datelike, Local};

use crate::states::medication::medication::Medication;
use crate::states::medication::record::Record;
use crate::states::medication::{self, schedule};
use crate::states::panel::Panel;
use crate::states::settings::Settings;
pub struct State {
    pub panel: Panel,
    pub settings: Settings,
    pub medications: Vec<Medication>,
    pub records: Vec<Record>,
}
impl State {
    pub fn new() -> Self {
        State {
            panel: Panel::Time,
            settings: Settings::new(),
            medications: Vec::new(),
            records: Vec::new(),
        }
    }
    pub fn change_panel(&mut self, panel: &Panel) {
        self.panel = panel.clone();
    }
    pub fn set_records(&mut self) {
        let medications: &Vec<Medication> = &self.medications;

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
                let time: [u8; 5] = [
                    chrono::Local::now().year() as u8,
                    chrono::Local::now().month() as u8,
                    chrono::Local::now().day() as u8,
                    schedule.time[0],
                    schedule.time[1],
                ];
                let record: Record = Record::new(medication_index, schedule_index, time);
                self.records.push(record);
            }
        }
    }
}
