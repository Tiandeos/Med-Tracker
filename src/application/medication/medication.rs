use chrono::{DateTime, Local, Utc};

use crate::application::medication::schedule::Schedule;

pub struct Medication {
    pub id: String, // Unique ID of medication
    pub name: String,
    pub stock: u32,
    pub created_at: DateTime<Utc>,
    pub schedules: Vec<Schedule>, // List of schedules of medication
}
impl Medication {
    pub fn new(name: String, stock: u32) -> Self {
        Medication {
            name,
            id: uuid::Uuid::new_v4().to_string(),
            stock,
            created_at: Local::now().to_utc(),
            schedules: Vec::new(),
        }
    }
}
