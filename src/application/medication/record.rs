use chrono::DateTime;

use super::occurrencestatus::OccurrenceStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub medication_id: String,       // UUID of medication.
    pub schedule_id: String,         // Schedule UUID of medication
    pub time: DateTime<chrono::Utc>, // Time of record
    pub occurrence_status: OccurrenceStatus,
}
impl Record {
    pub fn new(medication_id: String, schedule_id: String, time: DateTime<chrono::Utc>) -> Self {
        Record {
            id: uuid::Uuid::new_v4().to_string(),
            medication_id,
            schedule_id,
            time,
            occurrence_status: OccurrenceStatus::Pending,
        }
    }
    pub fn empty_new() -> Self {
        Record {
            id: uuid::Uuid::new_v4().to_string(),
            medication_id: String::new(),
            schedule_id: String::new(),
            time: chrono::Utc::now(),
            occurrence_status: OccurrenceStatus::Pending,
        }
    }
}
