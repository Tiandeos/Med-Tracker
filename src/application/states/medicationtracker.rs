use chrono::{DateTime, NaiveDate, Utc};

use crate::application::medication::{medication::Medication, occurrencestatus::OccurrenceStatus, record::Record};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
    pub fn mark_as_taken(&mut self, record_id: &str) {
        if let Some(record) = self.records.iter_mut().find(|r| r.id == record_id) {
            record.occurrence_status = OccurrenceStatus::Taken { taken_at: Utc::now() };
        }
    }

    pub fn mark_as_taken_at(&mut self, record_id: &str, taken_at: DateTime<Utc>) {
        if let Some(record) = self.records.iter_mut().find(|r| r.id == record_id) {
            record.occurrence_status = OccurrenceStatus::Taken { taken_at };
        }
    }
    pub fn mark_as_skipped(&mut self, record_id: &str) {
        if let Some(record) = self.records.iter_mut().find(|r| r.id == record_id) {
            record.occurrence_status = OccurrenceStatus::Skipped { reason: None };
        }
    }
    pub fn mark_as_missed(&mut self, record_id: &str) {
        if let Some(record) = self.records.iter_mut().find(|r| r.id == record_id) {
            record.occurrence_status = OccurrenceStatus::Missed;
        }
    }

    pub fn reschedule_record(&mut self, record_id: &str, new_time: DateTime<Utc>) {
        if let Some(record) = self.records.iter_mut().find(|r| r.id == record_id) {
            record.time = new_time;
        }
    }
}
