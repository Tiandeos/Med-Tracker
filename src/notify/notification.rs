pub fn send_alarm_notification(medication_name: &str, time_str: &str) {
    let body = format!("{} — scheduled at {}", medication_name, time_str);
    if let Err(e) = notify_rust::Notification::new()
        .summary("Med-Tracker")
        .body(&body)
        .show()
    {
        eprintln!("Failed to send notification: {e}");
    }
}
