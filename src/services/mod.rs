//! Here are all the Service related structs and traits

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::settings::WebSettings;

pub mod shortcuts;
pub mod web_settings;

/// The WebSettings service gives the option to get and load the web settings.
/// It can give them as a struct or as a "compiled" JavaScript object.
///
/// The service is used as an actor.
///
pub struct WebSettingsService {
    settings: WebSettings,
    compiled_settings: String,
}

/// This enum is used for getting and reloading the settings struct.
/// Reloading is done from disk. It uses the `settings.yaml`
pub enum WebSettingsMessage {
    Get,
    Reload,
}

/// "Compiled settings" are the JavaScript rendered version of the `settings.yaml`
pub enum WebSettingsCompiledMessage {
    Get,
    Reload,
}

/// The shortcuts are used in the frontend to give easier access to all the available functions in the frontend.
/// TODO: They can also be triggered by incoming MQTT events
pub struct ShortcutsService {
    shortcuts: HashMap<String, Vec<ShortcutData>>,
}

/// ShortcutData describes the data that is stored in a single shortcut "Task". As a shortcut can
/// contain multiple actions/events.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShortcutData {
    topic: String,
    payload: String,
    options: Value,
}

/// This trait is used to give the shortcuts type [HashMap<String, ShortcutData>] a way to easily
/// load and save the data to or from disk (shortcuts.yaml).
pub trait ShortcutReadWrite {
    fn load_shortcuts() -> Self;

    fn save_shortcuts(&self);

    fn single(&self, which: String) -> Self;
}

/// This enum provides all the options that the [ShortcutsService] is responding to.
pub enum ShortcutsMessage {
    List,
    Reload,
    Get(String),
    Add(String, Vec<ShortcutData>),
    Delete(String),
}
