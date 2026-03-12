use crate::ui::panel::{alarm, home, managemeds, record, settings};
use iced;
#[derive(Debug, Clone)]
pub enum Message {
    OpenTime,
    OpenManageMeds,
    OpenRecord,
    OpenSettings,
    Settings(settings::Message),
    Time(home::time::Message),
    Record(record::Message),
    ManageMeds(managemeds::Message),
    Alarm(alarm::Message),
    TimeCheck,
    TrayLeftClick,
    TrayRightClick { x: f64, y: f64 },
    TrayMenuShow,
    CloseRequested(iced::window::Id),
    WindowOpened(iced::window::Id),
    Quit,
    HideSidebar,
    OpenSidebar,
}
