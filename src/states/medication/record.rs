pub struct Record {
    pub schedule_index: usize, // Schedule of medication
    pub time: [u8; 5],         // Time of record [Year,Month,Day,Hour, Minute]
    pub is_taken: bool,        // Checks if medication is taken or not
    pub is_skipped: bool,      // Checks if medication is skipped or not
}
impl Record {
    pub fn new(schedule_index: usize, time: [u8; 5]) -> Self {
        Record {
            schedule_index,
            time,
            is_taken: false,
            is_skipped: false,
        }
    }
    pub fn empty_new() -> Self {
        Record {
            schedule_index: 0,
            time: [0; 5],
            is_taken: false,
            is_skipped: false,
        }
    }
    fn check_empty(&self) -> bool {
        self.time == [0; 5]
    }
}
