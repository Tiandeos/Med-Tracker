use chrono::DateTime;

use super::occurrencestatus::OccurrenceStatus;
pub struct Record {
    pub id: String,
    pub medication_id: String,       // ID of medication.
    pub schedule_index: usize,       // Schedule of medication
    pub time: DateTime<chrono::Utc>, // Time of record
    pub occurrence_status: OccurrenceStatus,
}
impl Record {
    pub fn new(medication_id: String, schedule_index: usize, time: DateTime<chrono::Utc>) -> Self {
        Record {
            id: uuid::Uuid::new_v4().to_string(),
            medication_id,
            schedule_index,
            time,
            occurrence_status: OccurrenceStatus::Pending,
        }
    }
    pub fn empty_new() -> Self {
        Record {
            id: uuid::Uuid::new_v4().to_string(),
            medication_id: String::new(),
            schedule_index: 0,
            time: chrono::Utc::now(),
            occurrence_status: OccurrenceStatus::Pending,
        }
    }
}
