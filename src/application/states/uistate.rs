use crate::ui::panel::{alarm, home, managemeds, medications, settings};
pub struct UIState {
    pub settingsui: settings::Settingsui,
    pub timeui: home::time::TimeUI,
    pub medicationsui: medications::medicationsmain::Record,
    pub recordui: managemeds::ManageMedsUI,
    pub alarmui: alarm::AlarmUI,
}
impl UIState {
    pub fn new() -> Self {
        UIState {
            settingsui: settings::Settingsui::new(),
            timeui: home::time::TimeUI::new(),
            medicationsui: medications::medicationsmain::Record::new(),
            recordui: managemeds::ManageMedsUI::new(),
            alarmui: alarm::AlarmUI::new(),
        }
    }
}
