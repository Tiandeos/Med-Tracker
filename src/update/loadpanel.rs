use crate::states::panel::Panel;
use crate::states::state::State;

pub fn load_panel(state: &mut State,panel: &Panel)
{
    state.change_panel(panel);
    println!("{:?}", state.panel);

}