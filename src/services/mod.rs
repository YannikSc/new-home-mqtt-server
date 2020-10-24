//! Here are all the Service related structs and traits

use std::collections::HashMap;

use actix::Addr;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::settings::WebSettings;

pub mod shortcuts;
pub mod web_settings;
pub mod dashboard;
pub mod group;

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

/// As an actor it takes care of dashboard actions
pub struct DashboardService {
    dashboards: Vec<DashboardData>,
}

/// Contains all dashboard relevant data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DashboardData {
    name: String,
    groups: Vec<String>,
}

/// This actor takes care of all Group and Group item transactions
pub struct GroupService {
    groups: Vec<GroupData>,
    dashboard: Addr<DashboardService>,
}

/// Contains all dashboard group data
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GroupData {
    name: String,
    size: i32,
    order: i32,
    items: Vec<GroupItemData>,
}

/// Contains information
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GroupItemData {
    name: String,
    #[serde(rename = "type")]
    item_type: String,
    data: Value,
}

/// This trait gives data structs a way to load and save its data from/to a file and also extracts
/// a named entity as a list as the API needs it
pub trait DataReadWrite {
    fn load() -> Self;

    fn save(&self);

    fn single(&self, which: String) -> Self;
}

/// This traits get the index (in a vector) or key (in a HashMap) for the given search term (or None)
trait IndexOf<I, T> {
    fn index_of(&self, search: T) -> Option<I>;
}


/// This enum provides all the options that the [ShortcutsService] is responding to.
/// All methods return a list of all results (except the Get)
pub enum ShortcutsMessage {
    /// Gets a list of all shortcuts
    List,

    /// Reloads the shortcuts from the shortcuts.yaml
    Reload,

    /// Gets the single shortcut
    Get(String),

    /// Adds a new shortcut with the given name as key in the yaml file
    Add(String, Vec<ShortcutData>),

    /// Deletes the given key from the yaml file
    Delete(String),
}

/// All the available Dashboard related actions are here
/// All methods return all (remaining/created) dashboards (except the Get)
pub enum DashboardMessage {
    /// Lists all available dashboards
    List,

    /// Reloads the dashboards from the yaml file
    Reload,

    /// Gets a single dashboard
    Get(String),

    /// Sets the given dashboard key to the given dashboard
    Set(String, DashboardData),

    /// Deletes the given dashboard from the yaml file
    Delete(String),
}


/// All the available group related actions are here
pub enum GroupMessage {
    /// Reloads the groups from the yaml file
    /// Returns an empty group
    Reload,

    /// Gets a single group
    Get(String),

    /// Sets the given group key to the given group
    Set(String, GroupData),

    /// Deletes the given group from the yaml file
    Delete(String),
}
