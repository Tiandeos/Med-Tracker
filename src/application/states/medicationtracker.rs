use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::application::medication::{medication::Medication, record::Record};
pub struct MedicationTracker {
    pub record_by_time: HashMap<String, Vec<Record>>,
    pub medications: Vec<Medication>,
}
impl MedicationTracker {
    pub fn new() -> Self {
        MedicationTracker {
            record_by_time: HashMap::new(),
            medications: Vec::new(),
        }
    }
    pub fn generate_records(
        &mut self,
        start_time: DateTime<chrono::Utc>,
        end_time: DateTime<chrono::Utc>,
    ) {
        for medication in self.medications.iter() {
            for schedule in medication.schedules.iter() {}
        }
    }
    pub fn mark_as_taken(&mut self, date: &str, occurrence_index: usize, taken_at: DateTime<Utc>) {}
    pub fn mark_as_skipped(&mut self, date: &str, occurrence_index: usize, skip_reason: String) {}
    pub fn mark_as_missed(&mut self, date: &str, occurrence_index: usize) {}
}
