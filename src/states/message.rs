use crate::states::panel::Panel;

#[derive(Debug, Clone)]
pub enum Message {
    OpenPanel(Panel)
}