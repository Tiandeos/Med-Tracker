use super::states::settings::Settings;
use super::states::state::State;
use super::states::uistate::UIState;
pub struct App {
    pub uistate: UIState,
    pub state: State,
    pub settings: Settings,
}
impl Default for App {
    fn default() -> Self {
        App {
            state: State::new(),
            settings: Settings::new(),
            uistate: UIState::new(),
        }
    }
}
