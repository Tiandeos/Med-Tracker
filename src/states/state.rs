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
    //fn check_periodic_schedule(&self) -> Record {}
    //fn check_weekday(&self, schedule: &Schedule, medication_index: &usize) -> Vec<Record> {}
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
