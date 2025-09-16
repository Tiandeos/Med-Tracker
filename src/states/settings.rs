use iced as ice;
pub struct Settings {
    pub language: String,
    pub is_theme_changed: bool,
    pub theme: ice::Theme,
    pub is_auto_startup: bool,
    pub is_minimize_to_tray: bool,
    pub is_24_hour_format: bool,
}
impl Settings {
    pub fn new() -> Settings {
        Settings {
            language: "English".to_string(),
            is_theme_changed: false,
            theme: ice::Theme::Dark,
            is_auto_startup: false,
            is_minimize_to_tray: false,
            is_24_hour_format: true,
        }
    }
}
impl Default for Settings {
    fn default() -> Self {
        Settings::new()
    }
}
