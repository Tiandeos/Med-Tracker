use crate::ui::panel::{alarm, managemeds, record, settings, time};
pub struct UIState {
    pub settingsui: settings::Settingsui,
    pub timeui: time::TimeUI,
    pub recordui: record::Record,
    pub managemedsui: managemeds::ManageMedsUI,
    pub alarmui: alarm::AlarmUI,
}
impl UIState {
    pub fn new() -> Self {
        UIState {
            settingsui: settings::Settingsui::new(),
            timeui: time::TimeUI::new(),
            recordui: record::Record::new(),
            managemedsui: managemeds::ManageMedsUI::new(),
            alarmui: alarm::AlarmUI::new(),
        }
    }
}
