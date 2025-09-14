use crate::states::panel::Panel;
use crate::ui::panel::{settings,time,record,managemeds};
pub struct State
{
    pub panel: Panel,
    pub settingsui: settings::Settingsui,
    pub timeui: time::Time,
    pub recordui: record::Record,
    pub managemedsui: managemeds::ManageMedsUI,
}
impl Default for State
{
    fn default() -> Self {
        State {
            panel: Panel::Time,
            settingsui: settings::Settingsui::new(),
            timeui: time::Time::new(),
            recordui: record::Record::new(),
            managemedsui: managemeds::ManageMedsUI::new(),
        }
    }

}
impl State
{
    pub fn change_panel(&mut self, panel: &Panel)
    {
        self.panel = panel.clone();
    }
}