use crate::ui::panel::{alarm, managemeds, record, settings, time};
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
    HideSidebar,
    OpenSidebar,
}
