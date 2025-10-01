pub struct Record {
    pub medication_index: usize, // Index of medication in medications list
    pub schedule_index: usize,   // Schedule of medication
    pub time: [u8; 5],           // Time of record [Year,Month,Day,Hour, Minute]
    pub is_taken: bool,          // Checks if medication is taken or not
    pub is_skipped: bool,        // Checks if medication is skipped or not
}
impl Record {
    pub fn new(medication_index: usize, schedule_index: usize, time: [u8; 5]) -> Self {
        Record {
            medication_index,
            schedule_index,
            time,
            is_taken: false,
            is_skipped: false,
        }
    }
}
