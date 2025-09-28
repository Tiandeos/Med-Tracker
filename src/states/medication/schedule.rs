use chrono::Weekday;

pub struct Schedule {
    pub time: [u32; 2],         // Hour, Minute
    pub week_day: Vec<Weekday>, // Available weekdays
}
impl Schedule {
    pub fn new(time: [u32; 2]) -> Self {
        Schedule {
            time,
            week_day: Vec::new(),
        }
    }
}
