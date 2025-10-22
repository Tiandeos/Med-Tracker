use super::occurrencestatus::OccurrenceStatus;
pub struct Record {
    pub medication_id: String, // ID of medication.
    pub schedule_index: usize, // Schedule of medication
    pub time: [u8; 5],         // Time of record [Year,Month,Day,Hour, Minute]
    pub occurrence_status: OccurrenceStatus,
}
impl Record {
    pub fn new(medication_id: String, schedule_index: usize, time: [u8; 5]) -> Self {
        Record {
            medication_id,
            schedule_index,
            time,
            occurrence_status: OccurrenceStatus::Pending,
        }
    }
    pub fn empty_new() -> Self {
        Record {
            medication_id: String::new(),
            schedule_index: 0,
            time: [0; 5],
            occurrence_status: OccurrenceStatus::Pending,
        }
    }
    fn check_empty(&self) -> bool {
        self.time == [0; 5]
    }
}
