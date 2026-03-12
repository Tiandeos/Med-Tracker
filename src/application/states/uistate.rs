use crate::ui::panel::{alarm, home, managemeds, record, settings};
pub struct UIState {
    pub settingsui: settings::Settingsui,
    pub timeui: home::time::TimeUI,
    pub recordui: record::Record,
    pub managemedsui: managemeds::ManageMedsUI,
    pub alarmui: alarm::AlarmUI,
}
impl UIState {
    pub fn new() -> Self {
        UIState {
            settingsui: settings::Settingsui::new(),
            timeui: home::time::TimeUI::new(),
            recordui: record::Record::new(),
            managemedsui: managemeds::ManageMedsUI::new(),
            alarmui: alarm::AlarmUI::new(),
        }
    }
}
