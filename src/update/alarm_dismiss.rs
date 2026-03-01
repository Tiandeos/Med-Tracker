use chrono::Utc;

use crate::application::states::medicationtracker::MedicationTracker;

pub fn dismiss_expired_alarms(
    tracker: &mut MedicationTracker,
    alarming_records: &mut Vec<String>,
) -> bool {
    let now = Utc::now();
    let expired: Vec<String> = alarming_records
        .iter()
        .filter(|id| {
            tracker
                .records
                .iter()
                .find(|r| &r.id == *id)
                .map(|r| now.signed_duration_since(r.time).num_minutes() > 15)
                .unwrap_or(true)
        })
        .cloned()
        .collect();
    for id in &expired {
        tracker.mark_as_missed(id);
        alarming_records.retain(|r| r != id);
    }
    !expired.is_empty()
}
