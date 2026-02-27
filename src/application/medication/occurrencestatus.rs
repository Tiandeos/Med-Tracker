use chrono::DateTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OccurrenceStatus {
    Pending,
    Taken { taken_at: DateTime<chrono::Utc> },
    Skipped { reason: Option<String> },
    Missed,
}
