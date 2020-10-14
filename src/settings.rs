use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSettings {
    #[serde(
        rename = "settings.mqtt_url",
        default = "WebSettings::default_mqtt_url"
    )]
    pub mqtt_url: String,

    #[serde(rename = "settings.mqtt_user", default)]
    pub mqtt_username: String,

    #[serde(rename = "settings.mqtt_pass", default)]
    pub mqtt_password: String,

    #[serde(
        rename = "settings.mqtt_url",
        default = "WebSettings::default_backend_url"
    )]
    pub backend_url: String,

    #[serde(rename = "settings.mqtt_user", default)]
    pub backend_user: String,

    #[serde(rename = "settings.mqtt_pass", default)]
    pub backend_pass: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    #[serde(default = "AppSettings::default_port")]
    pub port: u16,

    #[serde(default = "AppSettings::default_host")]
    pub host: String,

    #[serde(default)]
    pub server_type: ServerType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerType {
    Proxy(String),
    File(String),
}

impl WebSettings {
    pub fn default_mqtt_url() -> String {
        String::from("ws://127.0.0.1:9001")
    }

    pub fn default_backend_url() -> String {
        String::from("ws://127.0.0.1:9001")
    }
}

impl From<File> for WebSettings {
    fn from(file: File) -> Self {
        match serde_yaml::from_reader(file) {
            Ok(settings) => settings,
            Err(error) => {
                eprintln!("[ERROR] [Web Settings]: Could not build settings from file");
                eprintln!("{}", error);

                WebSettings::default()
            }
        }
    }
}

impl Default for WebSettings {
    fn default() -> Self {
        serde_yaml::from_str("{}").unwrap()
    }
}

impl AppSettings {
    pub fn default_port() -> u16 {
        9002
    }

    pub fn default_host() -> String {
        String::from("0.0.0.0")
    }

    pub fn load() -> Self {
        let shortcuts_path = current_dir().unwrap().join("settings.yaml");
        let file = OpenOptions::new().read(true).open(shortcuts_path);

        match file {
            Ok(file) => match serde_yaml::from_reader(file) {
                Ok(settings) => settings,
                Err(error) => {
                    eprintln!("[ERROR] [App Settings]: Could not parse file");
                    eprintln!("{}", error);

                    Default::default()
                }
            },
            Err(error) => {
                eprintln!("[WARN] [App Settings]: Could not open file");
                eprintln!("{}", error);

                Default::default()
            }
        }
    }

    pub fn save(&self) {
        let shortcuts_path = current_dir().unwrap().join("settings.yaml");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(shortcuts_path);

        match file {
            Ok(mut file) => {
                if let Err(error) = serde_yaml::to_writer(&file, &self) {
                    eprintln!("[ERROR] [App Settings]: Could not write shortcuts");
                    eprintln!("{}", error);

                    return;
                }

                if let Err(error) = file.flush() {
                    eprintln!("[WARN] [App Settings]: Could not fully write shortcuts");
                    eprintln!("{}", error);
                }
            }
            Err(error) => {
                eprintln!("[ERROR] [App Settings]: Could not open/create file");
                eprintln!("{}", error);
            }
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        serde_yaml::from_str("{}").unwrap()
    }
}

impl Default for ServerType {
    fn default() -> Self {
        ServerType::File(String::from("public"))
    }
}
