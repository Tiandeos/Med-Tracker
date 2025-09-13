use crate::states::panel::Panel;
use crate::ui::panel::settings;
use crate::ui::panel::time;
pub struct State
{
    pub panel: Panel,
    pub settingsui: settings::Settings,
    pub timeui: time::Time,
}
impl Default for State
{
    fn default() -> Self {
        State {
            panel: Panel::Time,
            settingsui: settings::Settings::new(),
            timeui: time::Time::new(),
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