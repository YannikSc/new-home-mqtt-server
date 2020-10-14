use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

pub struct ShortcutsService {
    shortcuts: HashMap<String, Vec<ShortcutData>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShortcutData {
    topic: String,
    payload: String,
    options: Value,
}

pub trait ShortcutReadWrite {
    fn load_shortcuts() -> Self;

    fn save_shortcuts(&self);

    fn single(&self, which: String) -> Self;
}

pub enum ShortcutsMessage {
    List,
    Get(String),
    Add(String, Vec<ShortcutData>),
    Delete(String),
}
