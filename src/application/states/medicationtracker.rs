use std::collections::HashMap;

use chrono::DateTime;

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
    }
}
