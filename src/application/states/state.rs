use std::collections::HashMap;

use chrono::Datelike;

use crate::application::medication::medication::Medication;
use crate::application::medication::record::Record;
use crate::application::medication::{self, schedule};
use crate::application::panel::Panel;
pub struct State {
    pub panel: Panel,
    pub medications: Vec<Medication>,
    pub records: HashMap<usize, Vec<Record>>,
}
impl State {
    pub fn new() -> Self {
        State {
            panel: Panel::Time,
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
