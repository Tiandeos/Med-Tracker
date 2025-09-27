pub struct Schedule {
    pub time: [u32; 2],    // Hour, Minute
    week_day: Vec<String>, // Available weekdays
}
impl Schedule {
    pub fn new(time: [u32; 2]) -> Self {
        Schedule {
            time,
            week_day: Vec::new(),
        }
    }
}
