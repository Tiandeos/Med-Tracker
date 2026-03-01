pub fn build_tray_icon() -> Option<tray_icon::TrayIcon> {
    tray_icon::TrayIconBuilder::new()
        .with_tooltip("Med-Tracker")
        .build()
        .map_err(|e| eprintln!("Failed to create tray icon: {e}"))
        .ok()
}
