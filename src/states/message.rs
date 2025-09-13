use crate::ui::panel::settings;
use crate::ui::panel::time;
#[derive(Debug, Clone)]
pub enum Message {
    OpenTime,
    OpenManageMeds,
    OpenRecord,
    OpenSettings,
    Settings(settings::Message),
    Time(time::Message),
}