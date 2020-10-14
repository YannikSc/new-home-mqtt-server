use crate::settings::WebSettings;

pub mod shortcuts;
pub mod web_settings;

pub struct WebSettingsService {
    settings: WebSettings,
    compiled_settings: String,
}

pub enum WebSettingsMessage {
    Get,
    Reload,
}

pub enum WebSettingsCompiledMessage {
    Get,
    Reload,
}

pub struct ShortcutsService {}
