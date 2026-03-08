use crate::application::message::Message;
use iced::futures::sink::SinkExt;
use iced::Subscription;
use std::time::Duration;
use tray::{MouseButton, MouseButtonState, TrayIconEvent};

pub fn tray_subscription(tray_active: bool) -> Subscription<Message> {
    if !tray_active {
        return Subscription::none();
    }
    Subscription::run(tray_stream)
}

fn tray_stream() -> impl iced::futures::Stream<Item = Message> {
    iced::stream::channel(16, async |mut output| {
        loop {
            tokio::time::sleep(Duration::from_millis(50)).await;
            while let Ok(event) = TrayIconEvent::receiver().try_recv() {
                match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        output.send(Message::TrayLeftClick).await.ok();
                    }
                    TrayIconEvent::Click {
                        button: MouseButton::Right,
                        button_state: MouseButtonState::Up,
                        position,
                        ..
                    } => {
                        output
                            .send(Message::TrayRightClick {
                                x: position.x,
                                y: position.y,
                            })
                            .await
                            .ok();
                    }
                    _ => {}
                }
            }
        }
    })
}
