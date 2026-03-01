use crate::application::message::Message;
use iced::Subscription;
use tray_icon::{MouseButton, TrayIconEvent};

pub fn tray_subscription() -> Subscription<Message> {
    Subscription::run(tray_stream)
}

fn tray_stream() -> impl futures::Stream<Item = Message> {
    futures::stream::unfold((), |_| async {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            if let Ok(event) = TrayIconEvent::receiver().try_recv() {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    ..
                } = event
                {
                    return Some((Message::TrayLeftClick, ()));
                }
            }
        }
    })
}
