//! This module implements all shortcut related structs and traits.

use std::collections::HashMap;
use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::Write;

use actix::{Actor, Context, Handler, Message, MessageResult};

use crate::services::{ShortcutData, DataReadWrite, ShortcutsMessage, ShortcutsService};

impl ShortcutsService {
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::<String, Vec<ShortcutData>>::load(),
        }
    }
}

impl Actor for ShortcutsService {
    type Context = Context<Self>;
}

impl Handler<ShortcutsMessage> for ShortcutsService {
    type Result = MessageResult<ShortcutsMessage>;

    fn handle(&mut self, msg: ShortcutsMessage, _: &mut Self::Context) -> Self::Result {
        match msg {
            ShortcutsMessage::List => MessageResult(self.shortcuts.clone()),
            ShortcutsMessage::Get(name) => MessageResult(self.shortcuts.single(name)),
            ShortcutsMessage::Add(name, data) => {
                self.shortcuts.insert(name, data);
                self.shortcuts.save();

                MessageResult(self.shortcuts.clone())
            }
            ShortcutsMessage::Delete(name) => {
                self.shortcuts.remove(&name);
                self.shortcuts.save();

                MessageResult(self.shortcuts.clone())
            }
            ShortcutsMessage::Reload => {
                self.shortcuts = HashMap::<String, Vec<ShortcutData>>::load();

                MessageResult(self.shortcuts.clone())
            }
        }
    }
}

impl Message for ShortcutsMessage {
    type Result = HashMap<String, Vec<ShortcutData>>;
}

impl DataReadWrite for HashMap<String, Vec<ShortcutData>> {
    fn load() -> Self {
        let shortcuts_path = current_dir().unwrap().join("shortcuts.yaml");
        let file = OpenOptions::new().read(true).open(shortcuts_path);

        match file {
            Ok(file) => match serde_yaml::from_reader(file) {
                Ok(shortcuts) => shortcuts,
                Err(error) => {
                    eprintln!("[ERROR] [Shortcuts]: Could not parse file");
                    eprintln!("{}", error);

                    HashMap::new()
                }
            },
            Err(error) => {
                eprintln!("[WARN] [Shortcuts]: Could not open file");
                eprintln!("{}", error);

                HashMap::new()
            }
        }
    }

    fn save(&self) {
        let shortcuts_path = current_dir().unwrap().join("shortcuts.yaml");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(shortcuts_path);

        match file {
            Ok(mut file) => {
                if let Err(error) = serde_yaml::to_writer(&file, &self) {
                    eprintln!("[ERROR] [Shortcuts]: Could not write shortcuts");
                    eprintln!("{}", error);

                    return;
                }

                if let Err(error) = file.flush() {
                    eprintln!("[WARN] [Shortcuts]: Could not fully write shortcuts");
                    eprintln!("{}", error);
                }
            }
            Err(error) => {
                eprintln!("[ERROR] [Shortcuts]: Could not open/create file");
                eprintln!("{}", error);
            }
        }
    }

    fn single(&self, which: String) -> Self {
        if let Some(data) = self.get(&which) {
            let mut map = HashMap::new();

            map.insert(which, data.clone());

            return map;
        }

        HashMap::new()
    }
}
