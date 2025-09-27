use crate::states::medication::medication::Medication;
use crate::states::panel::Panel;
use crate::states::settings::Settings;

pub struct State {
    pub panel: Panel,
    pub settings: Settings,
    pub medications: Vec<Medication>,
}
impl State {
    pub fn new() -> Self {
        State {
            panel: Panel::Time,
            settings: Settings::new(),
            medications: Vec::new(),
        }
    }
    pub fn change_panel(&mut self, panel: &Panel) {
        self.panel = panel.clone();
    }
}
