use crate::states::panel::Panel;
pub struct State
{
    pub panel: Panel,
}
impl Default for State
{
    fn default() -> Self {
        State {
            panel: Panel::Time,
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