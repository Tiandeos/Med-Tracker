use crate::application::states::medicationtracker::MedicationTracker;
use std::fs;
use std::io;
use std::path::PathBuf;

fn data_path() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join("med-tracker"))
}

pub fn save_tracker(tracker: &MedicationTracker) -> Result<(), io::Error> {
    let dir = data_path().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Could not find data directory",
    ))?;
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(tracker)?;
    fs::write(dir.join("data.json"), json)?;
    Ok(())
}
