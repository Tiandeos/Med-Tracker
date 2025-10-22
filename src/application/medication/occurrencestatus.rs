use chrono::DateTime;

#[derive(Debug, Clone)]
pub enum OccurrenceStatus {
    Pending,
    Taken { taken_at: DateTime<chrono::Utc> },
    Skipped { reason: Option<String> },
    Missed,
}
