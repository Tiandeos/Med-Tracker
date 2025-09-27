use crate::states::state::State;
use crate::states::uistate::UIState;
pub struct App {
    pub uistate: UIState,
    pub state: State,
}
impl Default for App {
    fn default() -> Self {
        App {
            state: State::new(),
            uistate: UIState::new(),
        }
    }
}
