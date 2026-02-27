use crate::application::states::medicationtracker::MedicationTracker;
use std::fs;
use std::path::PathBuf;

fn data_path() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join("med-tracker").join("data.json"))
}

pub fn load_tracker() -> Option<MedicationTracker> {
    let path = data_path()?;
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}
