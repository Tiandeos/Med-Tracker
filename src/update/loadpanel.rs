use crate::application::app::App;
use crate::application::panel::Panel;
pub fn load_panel(state: &mut App, panel: &Panel) {
    state.state.change_panel(panel);
    println!("{:?}", state.state.panel);
}
