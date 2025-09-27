use crate::states::app::App;
use crate::states::panel::Panel;
pub fn load_panel(state: &mut App, panel: &Panel) {
    state.state.change_panel(panel);
    println!("{:?}", state.state.panel);
}
