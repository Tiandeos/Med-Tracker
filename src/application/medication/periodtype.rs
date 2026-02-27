use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PeriodType {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}
