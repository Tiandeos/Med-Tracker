use chrono::{DateTime, NaiveDate, Utc};

use crate::application::medication::{medication::Medication, record::Record};
pub struct MedicationTracker {
    pub records: Vec<Record>,
    pub medications: Vec<Medication>,
    pub last_generation_date: Option<NaiveDate>,
}
impl MedicationTracker {
    pub fn new() -> Self {
        MedicationTracker {
            records: Vec::new(),
            medications: Vec::new(),
            last_generation_date: None,
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
