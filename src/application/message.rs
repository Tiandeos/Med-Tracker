use crate::ui::panel::{alarm, managemeds, record, settings, time};
use iced;
#[derive(Debug, Clone)]
pub enum Message {
    OpenTime,
    OpenManageMeds,
    OpenRecord,
    OpenSettings,
    Settings(settings::Message),
    Time(time::Message),
    Record(record::Message),
    ManageMeds(managemeds::Message),
    Alarm(alarm::Message),
    TimeCheck,
    TrayLeftClick,
    CloseRequested(iced::window::Id),
    Quit,
    HideSidebar,
    OpenSidebar,
}
