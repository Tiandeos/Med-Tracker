use chrono::Weekday;

pub struct Schedule {
    pub time: [u32; 2],         // Hour, Minute
    pub week_day: Vec<Weekday>, // Available weekdays
    pub is_completed: bool, // Checks if schedule has been finished and doesnt need to be checked
    pub is_skipped: bool, // Checks if this schedule skipped, sets completed true doesnt affect stocks.
}
impl Schedule {
    pub fn new(time: [u32; 2]) -> Self {
        Schedule {
            time,
            week_day: Vec::new(),
            is_completed: false,
            is_skipped: false,
        }
    }
}
