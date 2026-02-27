use crate::application::states::medicationtracker::MedicationTracker;
use std::fs;

pub fn load_tracker() -> Option<MedicationTracker> {
    let path = super::data_dir()?.join("data.json");
    let contents = fs::read_to_string(&path).ok()?;
    match serde_json::from_str(&contents) {
        Ok(tracker) => Some(tracker),
        Err(e) => {
            eprintln!("Failed to parse {}: {e}", path.display());
            None
        }
    }
}
