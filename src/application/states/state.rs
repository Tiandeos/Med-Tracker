use crate::application::panel::Panel;
pub struct State {
    pub panel: Panel,
    pub previous_panel: Option<Panel>,
}
impl State {
    pub fn new() -> Self {
        State {
            panel: Panel::Time,
            previous_panel: None,
        }
    }
    pub fn change_panel(&mut self, panel: &Panel) {
        self.panel = panel.clone();
    }
    pub fn switch_to_alarm(&mut self) {
        self.previous_panel = Some(self.panel.clone());
        self.panel = Panel::Alarm;
    }
    pub fn restore_previous_panel(&mut self) {
        if let Some(previous) = self.previous_panel.take() {
            self.panel = previous;
        }
    }
}
