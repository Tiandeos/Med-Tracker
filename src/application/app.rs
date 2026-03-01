use super::states::medicationtracker::MedicationTracker;
use iced;
use super::states::settings::Settings;
use super::states::state::State;
use super::states::uistate::UIState;
pub struct App {
    pub uistate: UIState,
    pub state: State,
    pub settings: Settings,
    pub medicationtracker: MedicationTracker,
    pub tray_icon: Option<tray_icon::TrayIcon>,
    pub window_id: Option<iced::window::Id>,
}
impl App {
    pub fn new() -> Self {
        App {
            state: State::new(),
            settings: Settings::new(),
            uistate: UIState::new(),
            medicationtracker: MedicationTracker::new(),
            tray_icon: None,
            window_id: None,
        }
    }
}
impl Default for App {
    fn default() -> Self {
        App {
            state: State::new(),
            settings: Settings::new(),
            uistate: UIState::new(),
            medicationtracker: MedicationTracker::new(),
            tray_icon: None,
            window_id: None,
        }
    }
}
