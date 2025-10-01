use super::periodtype::PeriodType;
use chrono::Weekday;

pub struct Schedule {
    pub time: [u8; 2],                   // Hour, Minute
    pub week_day: Option<Vec<Weekday>>,  // Available weekdays
    pub is_completed: bool, // Checks if schedule has been finished and doesnt need to be checked
    pub is_skipped: bool, // Checks if this schedule skipped, sets completed true doesnt affect stocks.
    pub period_type: Option<PeriodType>, // Period type of medication
    pub period_time: u8,
}
impl Schedule {
    pub fn new(time: [u8; 2], period_type: Option<PeriodType>, period_time: u8) -> Self {
        Schedule {
            time,
            week_day: None,
            is_completed: false,
            is_skipped: false,
            period_type,
            period_time,
        }
    }
    pub fn set_period_type(&mut self, period_type: PeriodType) {
        self.period_type = Some(period_type);
    }
    pub fn set_period_time(&mut self, time: u8) {
        self.period_time = time;
    }
}
