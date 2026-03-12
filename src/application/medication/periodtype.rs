use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum PeriodType {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

impl fmt::Display for PeriodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeriodType::Hourly => write!(f, "Hourly"),
            PeriodType::Daily => write!(f, "Daily"),
            PeriodType::Weekly => write!(f, "Weekly"),
            PeriodType::Monthly => write!(f, "Monthly"),
        }
    }
}
