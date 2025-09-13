use crate::states::panel::Panel;
use crate::ui::panel::settings;
pub struct State
{
    pub panel: Panel,
    pub settingsui: settings::Settings,
}
impl Default for State
{
    fn default() -> Self {
        State {
            panel: Panel::Time,
            settingsui: settings::Settings::new(),
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