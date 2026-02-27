use crate::application::states::medicationtracker::MedicationTracker;
use std::fs;
use std::io;

pub fn save_tracker(tracker: &MedicationTracker) -> Result<(), io::Error> {
    let dir = super::data_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Could not find data directory",
    ))?;
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(tracker)?;
    let tmp_path = dir.join("data.json.tmp");
    fs::write(&tmp_path, json)?;
    fs::rename(tmp_path, dir.join("data.json"))?;
    Ok(())
}
