use std::env;
use std::fs::{File, OpenOptions};

use actix::{Actor, Context, Handler, Message, MessageResult};

use crate::services::{WebSettingsCompiledMessage, WebSettingsMessage, WebSettingsService};
use crate::settings::WebSettings;

impl WebSettingsService {
    pub fn get_settings_file() -> std::io::Result<File> {
        let cwd = env::current_dir().unwrap();
        OpenOptions::new()
            .read(true)
            .open(cwd.join("web_settings.yaml"))
    }

    pub fn new() -> Self {
        WebSettingsService {
            settings: match Self::get_settings_file() {
                Ok(file) => WebSettings::from(file),
                _ => serde_yaml::from_str::<WebSettings>("{}").unwrap(),
            },
            compiled_settings: String::new(),
        }
    }

    fn compile_settings(&mut self) {
        match serde_json::to_string(&self.settings) {
            Ok(value) => {
                self.compiled_settings = format!("export default {}", value);
            }
            Err(error) => {
                eprintln!("Settings compile error: {:?}", error);
            }
        }
    }
}

impl Actor for WebSettingsService {
    type Context = Context<Self>;
}

impl Handler<WebSettingsMessage> for WebSettingsService {
    type Result = MessageResult<WebSettingsMessage>;

    fn handle(&mut self, msg: WebSettingsMessage, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            WebSettingsMessage::Reload => {
                self.settings = match Self::get_settings_file() {
                    Ok(file) => WebSettings::from(file),
                    _ => serde_yaml::from_str::<WebSettings>("{}").unwrap(),
                };
            }
            _ => {}
        }

        MessageResult(self.settings.clone())
    }
}

impl Handler<WebSettingsCompiledMessage> for WebSettingsService {
    type Result = MessageResult<WebSettingsCompiledMessage>;

    fn handle(
        &mut self,
        msg: WebSettingsCompiledMessage,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        match msg {
            WebSettingsCompiledMessage::Reload => {
                self.compile_settings();
            }
            _ => {}
        }

        MessageResult(self.compiled_settings.clone())
    }
}

impl Message for WebSettingsMessage {
    type Result = WebSettings;
}

impl Message for WebSettingsCompiledMessage {
    type Result = String;
}
