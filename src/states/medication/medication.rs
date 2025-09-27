use crate::states::schedule::Schedule;

pub struct Medication {
    name: String,            // Medication name
    stock: u32,              // Medication stock
    schedule: Vec<Schedule>, // List of schedules of medication
}
