use crate::application::message::Message;
use iced::Subscription;
use iced::futures::sink::SinkExt;
use ksni::TrayMethods;

struct WaylandTray {
    sender: tokio::sync::mpsc::UnboundedSender<Message>,
}

impl ksni::Tray for WaylandTray {
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn title(&self) -> String {
        "Med-Tracker".into()
    }

    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        let mut rgba = vec![0x2e_u8, 0xcc, 0x71, 0xff].repeat(24 * 24);
        for pixel in rgba.chunks_exact_mut(4) {
            pixel.rotate_right(1);
        }
        vec![ksni::Icon {
            width: 24,
            height: 24,
            data: rgba,
        }]
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        let _ = self.sender.send(Message::TrayLeftClick);
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::StandardItem;
        let show_sender = self.sender.clone();
        let quit_sender = self.sender.clone();
        vec![
            StandardItem {
                label: "Show Application".into(),
                activate: Box::new(move |_| {
                    let _ = show_sender.send(Message::TrayMenuShow);
                }),
                ..Default::default()
            }
            .into(),
            ksni::MenuItem::Separator,
            StandardItem {
                label: "Exit".into(),
                activate: Box::new(move |_| {
                    let _ = quit_sender.send(Message::Quit);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub fn wayland_tray_subscription() -> Subscription<Message> {
    Subscription::run(wayland_tray_stream)
}

fn wayland_tray_stream() -> impl iced::futures::Stream<Item = Message> {
    iced::stream::channel(16, async |mut output| {
        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<Message>();

        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                handle.spawn(async move {
                    let tray = WaylandTray { sender };
                    let _handle = match tray.spawn().await {
                        Ok(h) => h,
                        Err(e) => {
                            eprintln!("[tray-wayland] Failed to spawn SNI tray: {e}");
                            return;
                        }
                    };
                    std::future::pending::<()>().await;
                });
            }
            Err(e) => {
                eprintln!("[tray-wayland] No Tokio runtime available: {e}");
                return;
            }
        }

        while let Some(msg) = receiver.recv().await {
            if output.send(msg).await.is_err() {
                break;
            }
        }
    })
}
