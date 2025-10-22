use crate::application::panel::Panel;
pub struct State {
    pub panel: Panel,
}
impl State {
    pub fn new() -> Self {
        State { panel: Panel::Time }
    }
    pub fn change_panel(&mut self, panel: &Panel) {
        self.panel = panel.clone();
    }
}
