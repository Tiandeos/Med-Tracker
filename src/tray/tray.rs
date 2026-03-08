use tray::{Icon, TrayIcon, TrayIconBuilder};

fn placeholder_icon() -> Icon {
    let rgba = vec![0x2e_u8, 0xcc, 0x71, 0xff].repeat(24 * 24);
    Icon::from_rgba(rgba, 24, 24).expect("placeholder icon is valid")
}

pub fn is_wayland() -> bool {
    std::env::var("XDG_SESSION_TYPE")
        .map(|v| v.to_lowercase() == "wayland")
        .unwrap_or(false)
}

pub fn create_tray() -> Option<TrayIcon> {
    if is_wayland() {
        eprintln!("[tray] Wayland session detected — tray not supported, skipping.");
        return None;
    }

    let icon = placeholder_icon();

    match TrayIconBuilder::new()
        .with_tooltip("Med-Tracker")
        .with_icon(icon)
        .build()
    {
        Ok(tray) => Some(tray),
        Err(e) => {
            eprintln!("[tray] Failed to create tray icon: {e}");
            None
        }
    }
}
