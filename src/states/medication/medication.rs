use crate::states::medication::schedule::Schedule;

pub struct Medication {
    pub name: String,            // Medication name
    pub stock: u32,              // Medication stock
    pub schedule: Vec<Schedule>, // List of schedules of medication
}
impl Medication {
    pub fn new(name: String, stock: u32) -> Self {
        Medication {
            name,
            stock,
            schedule: Vec::new(),
        }
    }
}
